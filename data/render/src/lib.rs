#[macro_use]
extern crate serde_derive;

use std::error::Error;
use hubcaps::{Credentials, Github};
use chrono::{Utc, NaiveDateTime};

mod lints;
pub mod types;

use std::collections::BTreeMap;
use types::{Catalog, Entry, Tag, Tags, Type};

fn valid(entry: &Entry, tags: &Tags) -> Result<(), Box<dyn Error>> {
    let lints = [lints::name, lints::min_one_tag, lints::tags_existing];
    lints.iter().map(|lint| Ok(lint(&entry, &tags)?)).collect()
}

pub fn validate(tags: &Tags, entries: &Vec<Entry>) -> Result<(), Box<dyn Error>> {
    for entry in entries {
        valid(&entry, &tags)?
    }
    Ok(())
}

#[tokio::main]
pub async fn check_deprecated(token: std::string::String, entries: &mut Vec<Entry>) -> Result<Vec<Entry>, Box<dyn Error>> {
    let github = Github::new(
        String::from("user-agent-name"),
        Credentials::Token(token),
    )?;

    let mut entries_tmp: Vec<Entry> = entries.to_vec();
    for entry in &mut entries_tmp {
        if entry.source.is_none() {
            continue;
        }

        let mut source: &str = entry.source.as_ref().unwrap();
        if source.chars().last().unwrap() == '/' {
            source = source.trim_end_matches('/');
        }

        let components: Vec<&str> = source.split("/").collect();
        if !(components.contains(&"github.com") && components.len() == 5) {
            // valid github source must have 5 elements - anything longer and they are probably a
            // reference to a path inside a repo, rather than a repo itself.
            continue;
        }

        let owner = components[3];
        let repo  = components[4];

        let commit_list = github
            .repo(owner, repo)
            .commits()
            .list()
            .await;

        let commit_list = match commit_list {
            Ok(commit_list) => commit_list,
            Err(_error) => Vec::new(),
        };
        if commit_list.len() == 0 {
            continue;
        }

        let date: &str = &commit_list[0].commit.author.date[..];
        let timestamp = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%SZ")?.timestamp();
        let current_timestamp = Utc::now().timestamp();

        if current_timestamp - timestamp > 365 * 86400 {
            if entry.deprecated.is_none() {
                entry.deprecated = Some(true);
            }
        } else {
            if entry.deprecated.is_some() {
                entry.deprecated = None;
            }
        }
    }
    Ok(entries_tmp.to_vec())
}

pub fn group(tags: &Tags, entries: Vec<Entry>) -> Result<Catalog, Box<dyn Error>> {
    let mut linters = BTreeMap::new();

    // Move tools that support multiple languages into their own category
    let (multi, entries): (Vec<Entry>, Vec<Entry>) = entries.into_iter().partition(|entry| {
        entry.tags.len() > 1
            && entry.tags
                != vec!["c".to_string(), "cpp".to_string()]
                    .into_iter()
                    .collect()
    });

    let languages: Vec<&Tag> = tags
        .into_iter()
        .filter(|t| t.tag_type == Type::Language)
        .collect();

    for language in languages {
        let list: Vec<Entry> = entries
            .iter()
            .filter(|e| e.tags.contains(&language.tag))
            .map(|e| e.clone())
            .collect();
        if !list.is_empty() {
            linters.insert(language.clone(), list);
        }
    }

    let mut others = BTreeMap::new();
    let other_tags: Vec<&Tag> = tags
        .into_iter()
        .filter(|t| t.tag_type == Type::Other)
        .collect();
    for other in other_tags {
        let list: Vec<Entry> = entries
            .iter()
            .filter(|e| e.tags.contains(&other.tag))
            .map(|e| e.clone())
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
