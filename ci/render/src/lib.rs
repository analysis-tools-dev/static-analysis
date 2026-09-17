use anyhow::{Context, Result};
use chrono::{DateTime, Local, Utc};
use serde::Deserialize;
use slug::slugify;
use stats::StatsRaw;

/// Entry validation rules.
mod lints;
pub mod stats;
pub mod types;

use std::collections::BTreeMap;
use types::{Api, ApiEntry, Catalog, Collection, Entry, ParsedEntry, Tag, Type};

fn valid(entry: &ParsedEntry, tags: &[Tag]) -> Result<()> {
    let lints = [lints::name, lints::min_one_tag];
    lints.iter().try_for_each(|lint| lint(entry, tags))
}

#[derive(Deserialize)]
struct CommitResponse {
    commit: Commit,
}

#[derive(Deserialize)]
struct Commit {
    author: CommitAuthor,
}

#[derive(Deserialize)]
struct CommitAuthor {
    date: DateTime<Utc>,
}

fn github_coordinates(source: &str) -> Option<(&str, &str)> {
    let path = source
        .strip_prefix("https://github.com/")
        .or_else(|| source.strip_prefix("http://github.com/"))?
        .trim_end_matches('/');
    let (owner, repo) = path.split_once('/')?;
    (!owner.is_empty() && !repo.is_empty() && !repo.contains('/')).then_some((owner, repo))
}

async fn latest_commit_date(
    client: &reqwest::Client,
    token: &str,
    owner: &str,
    repo: &str,
) -> Result<Option<DateTime<Utc>>> {
    let url = format!("https://api.github.com/repos/{owner}/{repo}/commits?per_page=1");
    let response = client
        .get(&url)
        .bearer_auth(token)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .with_context(|| format!("Failed to fetch commits for {owner}/{repo}"))?;

    if matches!(
        response.status(),
        reqwest::StatusCode::NOT_FOUND | reqwest::StatusCode::CONFLICT
    ) {
        return Ok(None);
    }

    let commits = response
        .error_for_status()
        .with_context(|| format!("GitHub rejected the commits request for {owner}/{repo}"))?
        .json::<Vec<CommitResponse>>()
        .await
        .with_context(|| format!("Invalid commits response for {owner}/{repo}"))?;

    Ok(commits
        .into_iter()
        .next()
        .map(|commit| commit.commit.author.date))
}

/// Refreshes deprecation markers using each GitHub repository's latest commit.
///
/// # Errors
///
/// Returns an error when the HTTP client cannot be created or GitHub returns an
/// unexpected response.
pub async fn check_deprecated(token: &str, entries: &mut [Entry]) -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent("analysis-tools-render/0.2")
        .build()
        .context("Failed to build GitHub HTTP client")?;

    for entry in entries {
        let Some((owner, repo)) = entry.source.as_deref().and_then(github_coordinates) else {
            continue;
        };
        let last_commit = match latest_commit_date(&client, token, owner, repo).await {
            Ok(Some(date)) => date,
            Ok(None) => continue,
            Err(error) => {
                eprintln!("Could not check {owner}/{repo} for deprecation: {error:#}");
                continue;
            }
        };

        let duration = Local::now()
            .date_naive()
            .signed_duration_since(last_commit.date_naive());
        entry.deprecated = (duration.num_days() > 365).then_some(true);
    }

    Ok(())
}

/// Groups normalized entries for the generated README.
#[must_use]
pub fn create_catalog(
    entries: &[Entry],
    languages: &[Tag],
    other_tags: &[Tag],
    collections: Vec<Collection>,
) -> Catalog {
    // Multi-language tools get their own primary section instead of being repeated under
    // every language. They still belong in applicable non-language tag sections.
    let (multi, single_language): (Vec<Entry>, Vec<Entry>) =
        entries.iter().cloned().partition(|entry| {
            let language_tags = entry
                .tags
                .iter()
                .filter(|t| t.kind == Type::Language)
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
        collections,
    }
}

/// Converts normalized entries to the machine-readable API representation.
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

/// Converts raw page-view statistics into a tool-name lookup.
#[must_use]
pub fn format_stats(stats: StatsRaw) -> BTreeMap<String, String> {
    stats
        .data
        .result
        .into_iter()
        .map(|result| {
            (
                result.metric.path.trim_start_matches("/tool/").to_string(),
                result.value.1,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use askama::Template;
    use std::collections::BTreeSet;

    fn tag(name: &str, value: &str, kind: Type) -> Tag {
        Tag {
            name: name.into(),
            value: value.into(),
            kind,
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
    fn deprecated_tools_are_collapsed_in_every_section() -> Result<()> {
        let mut active = entry(&[]);
        active.name = "Active Tool".into();
        let mut deprecated = entry(&[]);
        deprecated.name = "Deprecated Tool".into();
        deprecated.deprecated = Some(true);
        deprecated.license = "proprietary".into();
        deprecated.discussion = Some("https://example.com/discussion".into());
        let tools = vec![deprecated, active];
        let catalog = Catalog {
            linters: BTreeMap::from([(tag("Rust", "rust", Type::Language), tools.clone())]),
            others: BTreeMap::from([(tag("Security", "security", Type::Other), tools.clone())]),
            multi: tools,
            collections: vec![],
        };
        let markdown = catalog.render()?;

        assert_eq!(
            markdown
                .matches("<summary>Show Deprecated</summary>")
                .count(),
            3
        );
        assert_eq!(
            markdown
                .matches("[Active Tool](https://example.com)")
                .count(),
            3
        );
        assert_eq!(markdown.matches("**Deprecated Tool**").count(), 3);
        assert!(!markdown.contains("[Deprecated Tool]("));
        assert!(!markdown.contains("<details open"));
        for heading in [
            "<h2>Rust</h2>",
            "## Multiple languages",
            "<h2>Security</h2>",
        ] {
            let section = markdown.split_once(heading).context("Missing section")?.1;
            let active_position = section
                .find("[Active Tool]")
                .context("Missing active tool")?;
            let details_position = section.find("<details>").context("Missing details")?;
            assert!(active_position < details_position);
            let hidden = section[details_position..]
                .split_once("</details>")
                .context("Unclosed details")?
                .0;
            assert!(hidden.contains("<summary>Show Deprecated</summary>\n\n- **Deprecated Tool**"));
            assert!(hidden.contains(
                "[:information_source:](<https://example.com/discussion>) :warning: :copyright:"
            ));
            assert!(!hidden.contains("[Active Tool]"));
        }
        Ok(())
    }

    #[test]
    fn no_empty_deprecated_sections_are_rendered() -> Result<()> {
        let mut explicitly_active = entry(&[]);
        explicitly_active.deprecated = Some(false);
        for tools in [vec![], vec![entry(&[])], vec![explicitly_active]] {
            let markdown = Catalog {
                linters: BTreeMap::new(),
                others: BTreeMap::new(),
                multi: tools,
                collections: vec![],
            }
            .render()?;
            assert!(!markdown.contains("Show Deprecated"));
        }
        Ok(())
    }

    #[test]
    fn deprecated_only_sections_keep_their_entries() -> Result<()> {
        let mut tool = entry(&[]);
        tool.deprecated = Some(true);
        let markdown = Catalog {
            linters: BTreeMap::new(),
            others: BTreeMap::new(),
            multi: vec![tool],
            collections: vec![],
        }
        .render()?;
        assert_eq!(
            markdown
                .matches("<summary>Show Deprecated</summary>")
                .count(),
            1
        );
        assert!(markdown.contains("\n\n- **Multi Tool** :warning: — Example tool\n\n</details>"));
        Ok(())
    }

    #[test]
    fn parses_github_repository_urls() {
        assert_eq!(
            github_coordinates("https://github.com/owner/repo"),
            Some(("owner", "repo"))
        );
        assert_eq!(
            github_coordinates("https://github.com/owner/repo/"),
            Some(("owner", "repo"))
        );
        assert_eq!(
            github_coordinates("https://github.com/owner/repo/tree/main"),
            None
        );
        assert_eq!(github_coordinates("https://gitlab.com/owner/repo"), None);
    }

    #[test]
    fn parses_github_commit_response() -> Result<()> {
        let response: Vec<CommitResponse> =
            serde_json::from_str(r#"[{"commit":{"author":{"date":"2026-08-01T12:34:56Z"}}}]"#)?;
        let date = response
            .into_iter()
            .next()
            .map(|commit| commit.commit.author.date);

        assert_eq!(
            date.map(|value| value.to_rfc3339()),
            Some("2026-08-01T12:34:56+00:00".into())
        );
        Ok(())
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

        let catalog = create_catalog(std::slice::from_ref(&tool), &languages, &other_tags, vec![]);

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
            vec![],
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
