use anyhow::{Result, ensure};

use crate::types::{ParsedEntry, Tag};

pub fn validate(entry: &ParsedEntry, tags: &[Tag]) -> Result<()> {
    name(entry, tags)?;
    min_one_tag(entry, tags)
}

pub fn name(entry: &ParsedEntry, _: &[Tag]) -> Result<()> {
    ensure!(
        entry.name.len() <= 50,
        "Name of entry may be at most 50 characters long, but {} is {} long",
        entry.name,
        entry.name.len()
    );
    Ok(())
}

pub fn min_one_tag(entry: &ParsedEntry, _: &[Tag]) -> Result<()> {
    ensure!(
        !entry.tags.is_empty(),
        "{} must have at least one tag from `tags.yml`.",
        entry.name
    );
    Ok(())
}
