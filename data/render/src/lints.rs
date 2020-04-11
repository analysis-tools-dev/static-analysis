use std::error::Error;

use crate::types::Entry;

pub fn filename(entry: &Entry) -> Result<(), Box<dyn Error>> {
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

pub fn min_one_tag(entry: &Entry) -> Result<(), Box<dyn Error>> {
    match entry.tags.is_empty() {
        true => Err(format!(
            "{} must have at least one tag from `categories.yml`.",
            entry.name
        )
        .into()),
        false => Ok(()),
    }
}
