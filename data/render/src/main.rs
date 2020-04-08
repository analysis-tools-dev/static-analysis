#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;

mod lints;
mod render;
mod types;

use render::render;
use std::collections::BTreeMap;
use types::{Catalog, Categories, Entry};

fn valid(entry: &Entry) -> Result<(), Box<dyn Error>> {
    let lints = [lints::filename, lints::min_one_tag];
    lints.iter().map(|lint| Ok(lint(&entry)?)).collect()
}

fn get_files() -> Result<(String, String), Box<dyn Error>> {
    let files: Vec<_> = env::args().skip(1).collect();
    if files.len() != 2 {
        return Err("Expected a two input files, `data.yml` and `categories.yml`".into());
    }
    Ok((files[0].clone(), files[1].clone()))
}

fn read_categories(file: String) -> Result<Categories, Box<dyn Error>> {
    let f = std::fs::File::open(file)?;
    Ok(serde_yaml::from_reader(f)?)
}

fn read_entries(file: String) -> Result<Vec<Entry>, Box<dyn Error>> {
    let f = std::fs::File::open(file)?;
    Ok(serde_yaml::from_reader(f)?)
}

fn validate(categories: &Categories, entries: &Vec<Entry>) -> Result<(), Box<dyn Error>> {
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

fn group(categories: &Categories, entries: Vec<Entry>) -> Result<Catalog, Box<dyn Error>> {
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

fn main() -> Result<(), Box<dyn Error>> {
    let (categories, data) = get_files()?;
    let categories = read_categories(categories)?;
    let mut entries = read_entries(data)?;
    entries.sort();
    validate(&categories, &entries)?;

    let catalog = group(&categories, entries)?;
    let template = std::fs::read_to_string("src/templates/README.md")?;
    let rendered = render(&template, catalog, categories)?;
    println!("{}", rendered);
    Ok(())
}
