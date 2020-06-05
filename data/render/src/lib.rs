#[macro_use]
extern crate serde_derive;

use std::error::Error;

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
