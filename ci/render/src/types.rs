use anyhow::{Result, bail};
use askama::Template;
use serde::{Deserialize, Serialize};
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
    #[serde(alias = "type", rename = "tag_type")]
    pub kind: Type,
    /// Include multi-language tools in this tag's rendered README section.
    #[serde(default, skip_serializing)]
    pub include_multi: bool,
}

// The tags from tags.yml. This remains a `Vec<Tag>` rather than a
// `BTreeSet<Tag>` so renders preserve the configured order.
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
    Gui,
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
    #[must_use]
    pub fn is_c_cpp(&self) -> bool {
        let language_tags = self.tags.iter().filter(|tag| tag.kind == Type::Language);

        language_tags.clone().count() == 2
            && language_tags
                .map(|tag| tag.value.as_str())
                .all(|value| matches!(value, "c" | "cpp"))
    }

    /// Whether the tool is marked as deprecated or unmaintained.
    #[must_use]
    pub fn is_deprecated(&self) -> bool {
        self.deprecated.unwrap_or(false)
    }

    /// Whether the tool uses the catalog's proprietary license marker.
    #[must_use]
    pub fn is_proprietary(&self) -> bool {
        self.license == "proprietary"
    }

    /// Validates and normalizes one parsed catalog entry.
    ///
    /// # Errors
    ///
    /// Returns an error when the entry fails validation or references an
    /// unknown tag or tool type.
    pub fn from_parsed(p: ParsedEntry, tags: &[Tag]) -> Result<Self> {
        valid(&p, tags)?;

        let tag_results: Vec<Result<Tag>> = p.tags.iter().map(|t| get_tag(t, tags)).collect();
        let tag_errors: Vec<String> = tag_results
            .iter()
            .filter_map(|r| r.as_ref().err().map(ToString::to_string))
            .collect();
        if !tag_errors.is_empty() {
            bail!(
                "Tool '{}': {}\n  File: data/tools/{}.yml",
                p.name,
                tag_errors.join("\n"),
                p.name.to_lowercase().replace(' ', "-")
            );
        }
        let entry_tags: Result<BTreeSet<Tag>> = tag_results.into_iter().collect();

        let types: Result<BTreeSet<ToolType>> = p
            .types
            .iter()
            .map(|t| {
                let value = serde_json::to_value(t)?;
                serde_json::from_value::<ToolType>(value).map_err(Into::into)
            })
            .collect();

        Ok(Self {
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
    bail!("Invalid tag: {t}")
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
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

impl Catalog {
    /// Arranges a tag map into three visually balanced table columns.
    fn rows(map: &EntryMap) -> Vec<Vec<(&Tag, &Vec<Entry>)>> {
        let num_columns = 3;
        let mut rows = Vec::new();
        let items: Vec<_> = map.iter().collect();
        let items_per_column = items.len().div_ceil(num_columns);

        for i in 0..items_per_column {
            let mut row = Vec::new();
            for col in 0..num_columns {
                let index = col * items_per_column + i;
                if index < items.len() {
                    row.push(items[index]);
                }
            }
            rows.push(row);
        }

        rows
    }

    #[must_use]
    pub fn linter_rows(&self) -> Vec<Vec<(&Tag, &Vec<Entry>)>> {
        Self::rows(&self.linters)
    }

    #[must_use]
    pub fn other_rows(&self) -> Vec<Vec<(&Tag, &Vec<Entry>)>> {
        Self::rows(&self.others)
    }
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
