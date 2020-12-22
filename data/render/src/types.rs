use askama::Template;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer,
};
use std::cmp::Ordering;
use std::{
    collections::{BTreeMap, HashSet},
    fmt::Display,
};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Type {
    #[serde(alias = "language")]
    Language,
    #[serde(alias = "other")]
    Other,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
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

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name.to_lowercase())
    }
}

struct TagVisitor;
impl<'de> Visitor<'de> for TagVisitor {
    type Value = Tag;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match s {
            _ => Ok(Tag::new("C", "c", Type::Language)),
        }
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(deserializer: D) -> Result<Tag, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(TagVisitor)
    }
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
            != [
                Tag::new("C", "c", Type::Language),
                Tag::new("C++", "cpp", Type::Language),
            ]
            .iter()
            .cloned()
            .collect::<HashSet<Tag>>()
    }
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
