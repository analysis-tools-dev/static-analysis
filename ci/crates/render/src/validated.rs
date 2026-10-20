use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, btree_set};
use std::fmt;
use std::ops::Deref;

/// A nonblank entry name of at most 50 UTF-8 bytes, stored without trimming.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct EntryName(String);

impl TryFrom<String> for EntryName {
    type Error = anyhow::Error;

    fn try_from(name: String) -> Result<Self> {
        ensure!(!name.trim().is_empty(), "Name of entry must not be blank");
        ensure!(
            name.len() <= 50,
            "Name of entry may be at most 50 UTF-8 bytes long, but {} is {} bytes long",
            name,
            name.len()
        );
        Ok(Self(name))
    }
}

impl AsRef<str> for EntryName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for EntryName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl fmt::Display for EntryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl From<EntryName> for String {
    fn from(name: EntryName) -> Self {
        name.0
    }
}

/// A nonempty, sorted set of entry tags, with no mutable access to its contents.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(try_from = "BTreeSet<T>")]
pub struct EntryTags<T: Ord = String>(BTreeSet<T>);

impl<T: Ord> TryFrom<BTreeSet<T>> for EntryTags<T> {
    type Error = anyhow::Error;

    fn try_from(tags: BTreeSet<T>) -> Result<Self> {
        ensure!(
            !tags.is_empty(),
            "Entry must have at least one tag from `tags.yml`."
        );
        Ok(Self(tags))
    }
}

impl<T: Ord> EntryTags<T> {
    /// Iterates over the tags in sorted order.
    pub fn iter(&self) -> btree_set::Iter<'_, T> {
        self.0.iter()
    }

    /// Whether this entry has the given tag.
    #[must_use]
    pub fn contains(&self, tag: &T) -> bool {
        self.0.contains(tag)
    }
}

impl<'a, T: Ord> IntoIterator for &'a EntryTags<T> {
    type Item = &'a T;
    type IntoIter = btree_set::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn name_construction_and_serde_reject_blank_and_overlong_names() -> Result<()> {
        for name in [
            String::new(),
            " \t\r\n".into(),
            "\u{2003}\u{a0}".into(),
            "a".repeat(51),
            "é".repeat(26),
        ] {
            assert!(EntryName::try_from(name.clone()).is_err());
            assert!(serde_json::from_value::<EntryName>(json!(name)).is_err());
            assert!(serde_saphyr::from_str::<EntryName>(&serde_json::to_string(&name)?).is_err());
        }
        Ok(())
    }

    #[test]
    fn names_preserve_whitespace_and_utf8_bytes_through_serde() -> Result<()> {
        for original in ["a".repeat(50), "é".repeat(25), "  Astrée \t".into()] {
            let name = EntryName::try_from(original.clone())?;
            assert_eq!(name.as_ref(), original);
            assert_eq!(name.to_string(), original);
            let encoded = serde_json::to_string(&name)?;
            assert_eq!(serde_json::from_str::<EntryName>(&encoded)?, name);
            assert_eq!(serde_saphyr::from_str::<EntryName>(&encoded)?, name);
            assert_eq!(serde_json::to_value(&name)?, json!(original));
            assert_eq!(String::from(name), original);
        }
        Ok(())
    }

    #[test]
    fn tags_reject_empty_sets_in_constructors_and_serde() {
        assert!(EntryTags::<String>::try_from(BTreeSet::new()).is_err());
        assert!(serde_json::from_str::<EntryTags>("[]").is_err());
        assert!(serde_saphyr::from_str::<EntryTags>("[]").is_err());
    }

    #[test]
    fn tags_are_sorted_and_deduplicated_in_constructors_and_serde() -> Result<()> {
        let tags = EntryTags::try_from(BTreeSet::from([
            String::from("rust"),
            String::from("cpp"),
            String::from("rust"),
        ]))?;
        assert!(tags.contains(&String::from("rust")));
        assert!(!tags.contains(&String::from("python")));
        assert_eq!(
            tags.iter().map(String::as_str).collect::<Vec<_>>(),
            ["cpp", "rust"]
        );
        assert_eq!(serde_json::to_value(&tags)?, json!(["cpp", "rust"]));
        assert_eq!(
            serde_json::from_value::<EntryTags>(json!(["rust", "cpp", "rust"]))?,
            tags
        );
        assert_eq!(
            serde_saphyr::from_str::<EntryTags>("[rust, cpp, rust]")?,
            tags
        );
        assert_eq!(
            serde_json::from_value::<EntryTags>(json!(["rust", "rust"]))?
                .iter()
                .count(),
            1
        );
        Ok(())
    }
}
