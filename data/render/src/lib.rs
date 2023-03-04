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

use std::{collections::BTreeMap, iter::FromIterator};
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

        let components: Vec<&str> = entry
            .source
            .as_ref()
            .unwrap()
            .trim_end_matches('/')
            .split('/')
            .collect();
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
            let last_commit_utc = DateTime::<Utc>::from_utc(last_commit, Utc);
            let duration = Local::today().signed_duration_since(last_commit_utc.date());

            if duration.num_days() > 365 {
                entry.deprecated = Some(true);
            } else {
                entry.deprecated = None;
            }
        }
    }

    Ok(())
}

pub fn create_catalog(entries: &[Entry], languages: &[Tag], other_tags: &[Tag]) -> Result<Catalog> {
    // Move tools that support multiple programming languages into their own category
    let (multi, entries): (Vec<Entry>, Vec<Entry>) = entries.iter().cloned().partition(|entry| {
        let language_tags = entry
            .tags
            .iter()
            .filter(|t| t.tag_type == Type::Language)
            .count();
        language_tags > 1 && !entry.is_c_cpp()
    });

    let mut linters = BTreeMap::new();
    for language in languages {
        let list: Vec<Entry> = entries
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
        let list: Vec<Entry> = entries
            .iter()
            .filter(|e| e.tags.contains(other))
            .cloned()
            .collect();
        if !list.is_empty() {
            others.insert(other.clone(), list);
        }
    }

    Ok(Catalog {
        linters,
        others,
        multi,
    })
}

pub fn create_api(catalog: Catalog, languages: &[Tag], other_tags: &[Tag]) -> Result<Api> {
    let mut api_entries = BTreeMap::new();

    // Concatenate all entries into one vector
    let mut entries: Vec<Entry> = Vec::from_iter(catalog.linters.into_values().flatten());
    entries.extend(Vec::from_iter(catalog.others.into_values().flatten()));
    entries.extend(catalog.multi);

    for entry in entries {
        // Get the language data for the entry. We iterate over all languages
        // and look up each language in the entry tags This is an O(n) operation
        // as we iterate over the language list only once while the lookup is an
        // O(1) operation thanks to the tag hash set.
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

    Ok(api_entries)
}

#[cfg(test)]
mod tests {
    use super::*;

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
