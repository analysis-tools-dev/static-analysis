use anyhow::{anyhow, Result};

use crate::types::Entry;
use crate::types::Tags;

pub fn name(entry: &Entry, _: &Tags) -> Result<()> {
    match entry.name.len() <= 50 {
        true => Ok(()),
        false => Err(anyhow!(
            "Name of entry may be at most 50 characters long, but {} is {} long",
            entry.name,
            entry.name.len()
        )),
    }
}

pub fn min_one_tag(entry: &Entry, _: &Tags) -> Result<()> {
    match entry.tags.is_empty() {
        true => Err(anyhow!("{} must have at least one tag from `tags.yml`.", entry.name)),
        false => Ok(()),
    }
}

pub fn tags_existing(entry: &Entry, tags: &Tags) -> Result<()> {
    for entry_tag in &entry.tags {
        if !tags.iter().any(|tag| &tag.tag == entry_tag) {
            return Err(anyhow!(
                "Unknown tag `{}` for entry `{}`. It might be missing from the `tags.yml` file.",
                entry_tag, entry.name
            ));
        }
    }
    Ok(())
}
