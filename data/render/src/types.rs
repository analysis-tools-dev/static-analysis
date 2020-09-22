use askama::Template;
use std::collections::{BTreeMap, HashSet};
use std::{cmp::Ordering, collections::HashMap};

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
    // The optional resources field is currently not parsed. It is only used
    // for the website. In the future we could extend
    // https://crates.io/crates/serde-tuple-vec-map to support optional fields
    // or implement our own deserializer.
    // Ideally we'd create a struct Resource { k: String, v: Url};
    #[serde(skip_deserializing)]
    pub resources: Option<Vec<(String, String)>>,
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
