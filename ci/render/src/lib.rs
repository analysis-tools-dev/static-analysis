#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use hubcaps::{Credentials, Github};
use slug::slugify;
use stats::StatsRaw;

mod lints;
pub mod stats;
pub mod types;

use std::collections::BTreeMap;
use types::{Api, ApiEntry, Catalog, Entry, ParsedEntry, Tag, Type};

fn valid(entry: &ParsedEntry, tags: &[Tag]) -> Result<()> {
    let lints = [lints::name, lints::min_one_tag];
    lints.iter().try_for_each(|lint| lint(entry, tags))
}

#[tokio::main]
pub async fn check_deprecated(token: String, entries: &mut Vec<Entry>) -> Result<()> {
    println!("Checking for deprecated entries on Github. This might take a while...");
    let github = Github::new(
        String::from("analysis tools bot"),
        Credentials::Token(token),
    )?;

    for entry in entries {
        if entry.source.is_none() {
            continue;
        }

        let Some(source) = entry.source.as_ref() else {
            continue;
        };
        let components: Vec<&str> = source.trim_end_matches('/').split('/').collect();
        if !(components.contains(&"github.com") && components.len() == 5) {
            // valid github source must have 5 elements - anything longer and they are probably a
            // reference to a path inside a repo, rather than a repo itself.
            continue;
        }

        let owner = components[3];
        let repo = components[4];

        if let Ok(commit_list) = github.repo(owner, repo).commits().list("").await {
            let date = &commit_list[0].commit.author.date;
            let last_commit = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%SZ")?;
            let last_commit_utc: DateTime<Utc> =
                DateTime::from_naive_utc_and_offset(last_commit, Utc);
            let now = Local::now().date_naive();
            let duration = now.signed_duration_since(last_commit_utc.date_naive());

            if duration.num_days() > 365 {
                entry.deprecated = Some(true);
            } else {
                entry.deprecated = None;
            }
        }
    }

    Ok(())
}

#[must_use]
pub fn create_catalog(entries: &[Entry], languages: &[Tag], other_tags: &[Tag]) -> Catalog {
    // Multi-language tools get their own primary section instead of being repeated under
    // every language. They still belong in applicable non-language tag sections.
    let (multi, single_language): (Vec<Entry>, Vec<Entry>) =
        entries.iter().cloned().partition(|entry| {
            let language_tags = entry
                .tags
                .iter()
                .filter(|t| t.tag_type == Type::Language)
                .count();
            language_tags > 1 && !entry.is_c_cpp()
        });

    let mut linters = BTreeMap::new();
    for language in languages {
        let list: Vec<Entry> = single_language
            .iter()
            .filter(|e| e.tags.contains(language))
            .cloned()
            .collect();
        if !list.is_empty() {
            linters.insert(language.clone(), list);
        }
    }

    let mut others = BTreeMap::new();
    for other in other_tags {
        let entries_for_tag: &[Entry] = if other.include_multi {
            entries
        } else {
            &single_language
        };
        let list: Vec<Entry> = entries_for_tag
            .iter()
            .filter(|e| e.tags.contains(other))
            .cloned()
            .collect();
        if !list.is_empty() {
            others.insert(other.clone(), list);
        }
    }

    Catalog {
        linters,
        others,
        multi,
    }
}

#[must_use]
pub fn create_api(entries: Vec<Entry>, languages: &[Tag], other_tags: &[Tag]) -> Api {
    let mut api_entries = BTreeMap::new();

    for entry in entries {
        // Get the language data for the entry. We iterate over all languages
        // and look up each language in the entry tags. This is an O(n) operation
        // as we iterate over the language list only once while the lookup is an
        // O(1) operation thanks to the tag set.
        let entry_languages = languages
            .iter()
            .filter_map(|lang| {
                if entry.tags.contains(lang) {
                    entry.tags.get(lang).map(|tag| tag.value.clone())
                } else {
                    None
                }
            })
            .collect();

        // ...same for the non-language tags
        let entry_other = other_tags
            .iter()
            .filter_map(|other| {
                if entry.tags.contains(other) {
                    entry.tags.get(other).map(|tag| tag.value.clone())
                } else {
                    None
                }
            })
            .collect();

        // In the future we want to split up licenses in the YAML input files into a list.
        // Emulate the future data format by creating a list from the current string.
        // Note that this string could contain more than one license name for now, e.g.
        // MIT / Apache License
        let licenses = vec![entry.license];

        let api_entry = ApiEntry {
            name: entry.name.clone(),
            categories: entry.categories,
            languages: entry_languages,
            other: entry_other,
            licenses,
            types: entry.types,
            homepage: entry.homepage,
            source: entry.source,
            pricing: entry.pricing,
            plans: entry.plans,
            description: entry.description,
            discussion: entry.discussion,
            deprecated: entry.deprecated,
            resources: entry.resources,
            reviews: entry.reviews,
            demos: entry.demos,
            wrapper: entry.wrapper,
        };
        api_entries.insert(slugify(&entry.name), api_entry);
    }

    api_entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn tag(name: &str, value: &str, tag_type: Type) -> Tag {
        Tag {
            name: name.into(),
            value: value.into(),
            tag_type,
            include_multi: false,
        }
    }

    fn entry(tags: &[Tag]) -> Entry {
        Entry {
            name: "Multi Tool".into(),
            categories: BTreeSet::new(),
            tags: tags.iter().cloned().collect(),
            license: "MIT".into(),
            types: BTreeSet::new(),
            homepage: "https://example.com".into(),
            source: None,
            pricing: None,
            plans: None,
            description: "Example tool".into(),
            discussion: None,
            deprecated: None,
            resources: None,
            reviews: None,
            demos: None,
            wrapper: None,
        }
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("this is a test"), "this-is-a-test".to_string());
        assert_eq!(slugify("Big"), "big".to_string());
        assert_eq!(slugify("   Big"), "big".to_string());
        assert_eq!(slugify("Astrée"), "astree".to_string());
        assert_eq!(slugify("non word 1234"), "non-word-1234".to_string());
        assert_eq!(slugify("it-has-dashes"), "it-has-dashes".to_string());
        assert_eq!(
            slugify("   - - it-has-dashes - -"),
            "it-has-dashes".to_string()
        );
    }

    #[test]
    fn multi_language_tools_remain_visible_in_other_sections_and_api() {
        let python = tag("Python", "python", Type::Language);
        let rust = tag("Rust", "rust", Type::Language);
        let mut ai_generated = tag("AI-generated code", "ai-generated-code", Type::Other);
        ai_generated.include_multi = true;
        let tool = entry(&[python.clone(), rust.clone(), ai_generated.clone()]);
        let languages = [python, rust];
        let other_tags = [ai_generated.clone()];

        let catalog = create_catalog(std::slice::from_ref(&tool), &languages, &other_tags);

        assert!(catalog.linters.is_empty());
        assert_eq!(catalog.multi.len(), 1);
        assert_eq!(catalog.multi[0], tool);
        assert_eq!(catalog.others[&ai_generated].len(), 1);
        assert_eq!(catalog.others[&ai_generated][0], tool);

        let api = create_api(vec![tool], &languages, &other_tags);
        assert_eq!(api["multi-tool"].languages, ["python", "rust"]);
        assert_eq!(api["multi-tool"].other, ["ai-generated-code"]);
    }

    #[test]
    fn c_and_cpp_tools_stay_in_language_sections_when_they_have_other_tags() {
        let c = tag("C", "c", Type::Language);
        let cpp = tag("C++", "cpp", Type::Language);
        let security = tag("Security/SAST", "security", Type::Other);
        let tool = entry(&[c.clone(), cpp.clone(), security.clone()]);

        let catalog = create_catalog(
            std::slice::from_ref(&tool),
            &[c.clone(), cpp.clone()],
            std::slice::from_ref(&security),
        );

        assert!(catalog.multi.is_empty());
        assert_eq!(catalog.linters[&c].len(), 1);
        assert_eq!(catalog.linters[&c][0], tool);
        assert_eq!(catalog.linters[&cpp].len(), 1);
        assert_eq!(catalog.linters[&cpp][0], tool);
        assert_eq!(catalog.others[&security].len(), 1);
        assert_eq!(catalog.others[&security][0], tool);
    }
}

pub fn format_stats(stats: StatsRaw) -> BTreeMap<String, String> {
    stats
        .data
        .result
        .into_iter()
        .map(|r| {
            (
                r.metric.path.trim_start_matches("/tool/").to_string(),
                r.value.1,
            )
        })
        .collect()
}
