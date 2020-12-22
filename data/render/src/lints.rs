use anyhow::{anyhow, Result};

use crate::types::ParsedEntry;
use crate::types::Tag;

pub fn name(entry: &ParsedEntry, _: &[Tag]) -> Result<()> {
    match entry.name.len() <= 50 {
        true => Ok(()),
        false => Err(anyhow!(
            "Name of entry may be at most 50 characters long, but {} is {} long",
            entry.name,
            entry.name.len()
        )),
    }
}

pub fn min_one_tag(entry: &ParsedEntry, _: &[Tag]) -> Result<()> {
    match entry.tags.is_empty() {
        true => Err(anyhow!(
            "{} must have at least one tag from `tags.yml`.",
            entry.name
        )),
        false => Ok(()),
    }
}
