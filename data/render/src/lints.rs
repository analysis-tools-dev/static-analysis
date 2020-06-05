use std::error::Error;

use crate::types::Entry;
use crate::types::Tags;

pub fn name(entry: &Entry, _: &Tags) -> Result<(), Box<dyn Error>> {
    match entry.name.len() <= 50 {
        true => Ok(()),
        false => Err(format!(
            "Name of entry may be at most 50 characters long, but {} is {} long",
            entry.name,
            entry.name.len()
        )
        .into()),
    }
}

pub fn min_one_tag(entry: &Entry, _: &Tags) -> Result<(), Box<dyn Error>> {
    match entry.tags.is_empty() {
        true => Err(format!("{} must have at least one tag from `tags.yml`.", entry.name).into()),
        false => Ok(()),
    }
}

pub fn tags_existing(entry: &Entry, tags: &Tags) -> Result<(), Box<dyn Error>> {
    for entry_tag in &entry.tags {
        if !tags.iter().any(|tag| &tag.tag == entry_tag) {
            return Err(format!(
                "Unknown tag `{}` for entry `{}`. It might be missing from the `tags.yml` file.",
                entry_tag, entry.name
            )
            .into());
        }
    }
    Ok(())
}
