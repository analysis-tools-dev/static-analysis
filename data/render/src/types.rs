use anyhow::{bail, Result};
use askama::Template;
use serde::Deserialize;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

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
    pub value: String,
    #[serde(alias = "type")]
    pub tag_type: Type,
}

impl Tag {
    fn new(name: &str, value: &str, tag_type: Type) -> Tag {
        Tag {
            name: name.into(),
            value: value.into(),
            tag_type,
        }
    }
}

// The tags from tags.yml. Note that this is a `Vector<Tag>` and not a
// `BTreeSet<Tag>` because we like to keep the sorting between renders.
pub type Tags = Vec<Tag>;

pub type EntryTags = BTreeSet<String>;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Resource {
    title: String,
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Review {
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Demo {
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[serde(rename = "category")]
pub enum Category {
    #[serde(rename = "linter")]
    Linter,
    #[serde(rename = "formatter")]
    Formatter,
    #[serde(rename = "performance")]
    Performance,
    #[serde(rename = "meta")]
    Meta,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParsedEntry {
    pub name: String,
    pub categories: BTreeSet<Category>,
    pub tags: BTreeSet<String>,
    pub license: String,
    pub types: BTreeSet<String>,
    pub homepage: String,
    pub source: Option<String>,
    pub pricing: Option<String>,
    pub plans: Option<BTreeMap<String, bool>>,
    pub description: String,
    pub discussion: Option<String>,
    pub deprecated: Option<bool>,
    pub resources: Option<Vec<Resource>>,
    pub reviews: Option<BTreeSet<String>>,
    pub demos: Option<BTreeSet<String>>,
    pub wrapper: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd)]
pub enum ToolType {
    #[serde(rename = "cli")]
    Commandline,
    #[serde(rename = "gui")]
    GUI,
    #[serde(rename = "service")]
    Service,
    #[serde(rename = "ide-plugin")]
    IdePlugin,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    pub categories: BTreeSet<Category>,
    pub tags: BTreeSet<Tag>,
    pub license: String,
    pub types: BTreeSet<ToolType>,
    pub homepage: String,
    pub source: Option<String>,
    pub pricing: Option<String>,
    pub plans: Option<BTreeMap<String, bool>>,
    pub description: String,
    pub discussion: Option<String>,
    pub deprecated: Option<bool>,
    pub resources: Option<Vec<Resource>>,
    pub reviews: Option<BTreeSet<String>>,
    pub demos: Option<BTreeSet<String>>,
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
            .collect::<BTreeSet<Tag>>()
    }

    pub fn from_parsed(p: ParsedEntry, tags: &[Tag]) -> Result<Entry> {
        valid(&p, tags)?;
        let entry_tags: Result<BTreeSet<Tag>> = p.tags.iter().map(|t| get_tag(t, tags)).collect();
        let types: Result<BTreeSet<ToolType>> = p
            .types
            .iter()
            .map(|t| {
                serde_json::from_value::<ToolType>(serde_json::to_value(t).unwrap())
                    .map_err(|e| e.into())
            })
            .collect();

        Ok(Entry {
            name: p.name,
            categories: p.categories,
            tags: entry_tags?,
            license: p.license,
            types: types?,
            homepage: p.homepage,
            source: p.source,
            pricing: p.pricing,
            plans: p.plans,
            description: p.description,
            discussion: p.discussion,
            deprecated: p.deprecated,
            resources: p.resources,
            reviews: p.reviews,
            demos: p.demos,
            wrapper: p.wrapper,
        })
    }
}

fn get_tag(t: &str, tags: &[Tag]) -> Result<Tag> {
    for tag in tags {
        if tag.value == t {
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

/// An entry of the machine-readable JSON out from the tool.
///
/// We use a different, de-normalized data format instead of the catalog, which
/// keeps the information for each tool in a struct instead of grouping tools by
/// tags.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiEntry {
    /// The original entry name (not slugified)
    pub name: String,
    pub categories: BTreeSet<Category>,
    pub languages: Vec<String>,
    pub other: Vec<String>,
    pub licenses: Vec<String>,
    pub types: BTreeSet<ToolType>,
    pub homepage: String,
    pub source: Option<String>,
    pub pricing: Option<String>,
    pub plans: Option<BTreeMap<String, bool>>,
    pub description: String,
    pub discussion: Option<String>,
    pub deprecated: Option<bool>,
    pub resources: Option<Vec<Resource>>,
    pub reviews: Option<BTreeSet<String>>,
    pub demos: Option<BTreeSet<String>>,
    pub wrapper: Option<bool>,
}

/// The final API dataformat is a map where the key is the entry name and the
/// value is the entry data, which makes searching for a tool's data easier
pub type Api = BTreeMap<String, ApiEntry>;
