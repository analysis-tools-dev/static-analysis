#[macro_use]
extern crate serde_derive;

use anyhow::Result;
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use hubcaps::{Credentials, Github};

mod lints;
pub mod types;

use std::collections::BTreeMap;
use types::{Catalog, Entry, ParsedEntry, Tag, Type};

fn valid(entry: &ParsedEntry, tags: &[Tag]) -> Result<()> {
    let lints = [lints::name, lints::min_one_tag];
    lints.iter().try_for_each(|lint| Ok(lint(entry, tags)?))
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

pub fn group(tags: &[Tag], entries: &[Entry]) -> Result<Catalog> {
    let mut linters = BTreeMap::new();

    // Move tools that support multiple programming languages into their own category
    let (multi, entries): (Vec<Entry>, Vec<Entry>) = entries.iter().cloned().partition(|entry| {
        let language_tags = entry
            .tags
            .iter()
            .filter(|t| t.tag_type == Type::Language)
            .count();
        language_tags > 1 && !entry.is_c_cpp()
    });

    let languages: Vec<&Tag> = tags
        .iter()
        .filter(|t| t.tag_type == Type::Language)
        .collect();

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
    let other_tags: Vec<&Tag> = tags.iter().filter(|t| t.tag_type == Type::Other).collect();
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
