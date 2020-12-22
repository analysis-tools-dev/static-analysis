use anyhow::{bail, Result};
use askama::Template;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

use crate::valid;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Type {
    #[serde(alias = "language")]
    Language,
    #[serde(alias = "other")]
    Other,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Tag {
    pub name: String,
    pub tag: String,
    #[serde(alias = "type")]
    pub tag_type: Type,
}

impl Tag {
    fn new(name: &str, tag: &str, tag_type: Type) -> Tag {
        Tag {
            name: name.into(),
            tag: tag.into(),
            tag_type,
        }
    }
}

// The tags from tags.yml. Note that this is a `Vector<Tag>` and not a
// `HashSet<Tag>` because we like to keep the sorting between renders.
pub type Tags = Vec<Tag>;

pub type EntryTags = HashSet<String>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resource {
    title: String,
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParsedEntry {
    pub name: String,
    pub categories: HashSet<String>,
    pub tags: HashSet<String>,
    pub license: String,
    pub types: HashSet<String>,
    pub homepage: String,
    pub source: Option<String>,
    pub description: String,
    pub discussion: Option<String>,
    pub deprecated: Option<bool>,
    pub resources: Option<Vec<Resource>>,
    pub wrapper: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub categories: HashSet<String>,
    pub tags: HashSet<Tag>,
    pub license: String,
    pub types: HashSet<String>,
    pub homepage: String,
    pub source: Option<String>,
    pub description: String,
    pub discussion: Option<String>,
    pub deprecated: Option<bool>,
    pub resources: Option<Vec<Resource>>,
    pub wrapper: Option<bool>,
}

impl Entry {
    pub fn is_c_cpp(&self) -> bool {
        self.tags
            == [
                Tag::new("C", "c", Type::Language),
                Tag::new("C++", "cpp", Type::Language),
            ]
            .iter()
            .cloned()
            .collect::<HashSet<Tag>>()
    }

    pub fn from_parsed(p: ParsedEntry, tags: &[Tag]) -> Result<Entry> {
        valid(&p, tags)?;
        let entry_tags: Result<HashSet<Tag>> = p.tags.iter().map(|t| get_tag(t, tags)).collect();
        Ok(Entry {
            name: p.name,
            categories: p.categories,
            tags: entry_tags?,
            license: p.license,
            types: p.types,
            homepage: p.homepage,
            source: p.source,
            description: p.description,
            discussion: p.discussion,
            deprecated: p.deprecated,
            resources: p.resources,
            wrapper: p.wrapper,
        })
    }
}

fn get_tag(t: &str, tags: &[Tag]) -> Result<Tag> {
    for tag in tags {
        if tag.tag == t {
            return Ok(tag.clone());
        }
    }
    bail!("Invalid tag: {}", t)
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

pub type EntryMap = BTreeMap<Tag, Vec<Entry>>;

#[derive(Debug, Serialize, Deserialize, Template)]
#[template(path = "README.md")]
pub struct Catalog {
    pub linters: EntryMap,
    pub others: EntryMap,
    pub multi: Vec<Entry>,
}
