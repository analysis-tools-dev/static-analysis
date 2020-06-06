use askama::Template;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

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
    pub homepage: String,
    pub source: Option<String>,
    pub description: String,
    pub tags: EntryTags,
    pub proprietary: Option<bool>,
    pub deprecated: Option<bool>,
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

/// any filters defined in `mod filters` are accessible in templates
mod filters {
    // eventually, if other open-source sites (e.g. gitlab) support something like stars, those
    // could also be formatted in this filter
    pub fn format_badge(s: &str) -> ::askama::Result<String> {
        let components: Vec<&str> = s.split("/").collect();
        if components.contains(&"github.com") && components.len() == 5 {
            // valid github source must have 5 elements - anything longer and they are probably a
            // reference to a path inside a repo, rather than a repo itself.
            Ok(format!("![stars]({}{}) ",
                // shields.io can't have a trailing '/' before parameters begin
                s.replace("github.com", "img.shields.io/github/stars").trim_end_matches("/"),
                "?style=flat-square&color=ccc",
            ))
        } else {
            Ok("".to_string())
        }
    }
}
