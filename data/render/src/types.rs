use askama::Template;
use std::collections::{BTreeMap, HashSet};
use std::cmp::Ordering;

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

// The tags from tags.yml Note that this is a `Vector<Tag>` and not a
// `HashSet<Tag>` because we like to keep the sorting between renders.
pub type Tags = Vec<Tag>;

pub type EntryTags = HashSet<String>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resource {
    title: String,
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
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
