#[macro_use]
extern crate serde_derive;

use std::error::Error;

mod lints;
pub mod types;

use std::collections::BTreeMap;
use types::{Catalog, Categories, Entry};

use tera::{Context, Tera};

fn valid(entry: &Entry) -> Result<(), Box<dyn Error>> {
    let lints = [lints::filename, lints::min_one_tag];
    lints.iter().map(|lint| Ok(lint(&entry)?)).collect()
}

pub fn validate(categories: &Categories, entries: &Vec<Entry>) -> Result<(), Box<dyn Error>> {
    for entry in entries {
        for tag in &entry.tags {
            if !categories.contains(tag) {
                return Err(format!(
                "Unknown tag `{}` for entry `{}`. It might be missing from the `categories.yml` file.",
                tag , entry.name
            )
                .into());
            }
        }
        if let Err(e) = valid(&entry) {
            return Err(e);
        }
    }
    Ok(())
}

pub fn group(categories: &Categories, entries: Vec<Entry>) -> Result<Catalog, Box<dyn Error>> {
    let mut linters = BTreeMap::new();

    // Move tools that support multiple languages into their own category
    let (multi, entries): (Vec<Entry>, Vec<Entry>) = entries.into_iter().partition(|entry| {
        entry.tags.len() > 1
            && entry.tags
                != ["c".to_string(), "cpp".to_string()]
                    .iter()
                    .cloned()
                    .collect()
    });

    for language in &categories.languages {
        let list: Vec<Entry> = entries
            .iter()
            .filter(|e| e.tags.contains(&language.tag))
            .map(|e| e.clone())
            .collect();
        if !list.is_empty() {
            linters.insert(language.name.clone(), list);
        }
    }

    let mut others = BTreeMap::new();
    for other in &categories.other {
        let list: Vec<Entry> = entries
            .iter()
            .filter(|e| e.tags.contains(&other.tag))
            .map(|e| e.clone())
            .collect();
        if !list.is_empty() {
            others.insert(other.name.clone(), list);
        }
    }

    Ok(Catalog {
        linters,
        others,
        multi,
    })
}

pub fn render(
    template: &str,
    catalog: Catalog,
    categories: Categories,
) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("catalog", &catalog);
    context.insert("categories", &categories);
    Ok(Tera::one_off(template, &context, true)?)
}
