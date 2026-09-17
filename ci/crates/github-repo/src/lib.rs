//! Shared tool source and GitHub repository URL classification.

use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A repository parsed from a GitHub HTTP(S) URL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GithubRepo {
    owner: String,
    name: String,
    original_url: String,
}

impl GithubRepo {
    /// Returns the original URL without normalizing its spelling or trailing slashes.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.original_url
    }
}

impl TryFrom<&str> for GithubRepo {
    type Error = anyhow::Error;

    /// Parses an HTTP(S) GitHub repository URL, ignoring trailing slashes.
    ///
    /// # Errors
    ///
    /// Returns an error for an unsupported prefix, missing owner or repository,
    /// or a repository path containing a subpath.
    fn try_from(original_url: &str) -> Result<Self> {
        let url = original_url.trim_end_matches('/');
        let path = url
            .strip_prefix("https://github.com/")
            .or_else(|| url.strip_prefix("http://github.com/"))
            .context("Expected a GitHub HTTP(S) URL")?;
        let (owner, name) = path.split_once('/').context("Expected owner/repository")?;
        ensure!(
            !owner.is_empty() && !name.is_empty() && !name.contains('/'),
            "Expected a repository URL with no subpath"
        );
        Ok(Self {
            owner: owner.to_owned(),
            name: name.to_owned(),
            original_url: original_url.to_owned(),
        })
    }
}

impl std::fmt::Display for GithubRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

/// A tool's source URL, classified without rejecting unsupported or malformed URLs.
///
/// Serialized as the original plain string, not as a tagged enum.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolSource {
    /// A supported GitHub repository URL.
    Github(GithubRepo),
    /// Any other source string, retained verbatim for manual review.
    Other(String),
}

impl ToolSource {
    /// Returns the original source string without URL normalization.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Github(repo) => repo.as_str(),
            Self::Other(source) => source,
        }
    }
}

impl From<String> for ToolSource {
    fn from(source: String) -> Self {
        GithubRepo::try_from(source.as_str()).map_or(Self::Other(source), Self::Github)
    }
}

impl From<&str> for ToolSource {
    fn from(source: &str) -> Self {
        source.to_owned().into()
    }
}

impl std::fmt::Display for ToolSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Serialize for ToolSource {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ToolSource {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_github_url() -> Result<()> {
        let repo = GithubRepo::try_from("https://github.com/owner/repo")?;
        assert_eq!(repo.owner, "owner");
        assert_eq!(repo.name, "repo");
        assert_eq!(repo.as_str(), "https://github.com/owner/repo");
        assert_eq!(repo.to_string(), "owner/repo");
        Ok(())
    }

    #[test]
    fn parses_trailing_slash() -> Result<()> {
        let repo = GithubRepo::try_from("https://github.com/owner/repo/")?;
        assert_eq!(repo.to_string(), "owner/repo");
        assert_eq!(repo.as_str(), "https://github.com/owner/repo/");
        Ok(())
    }

    #[test]
    fn preserves_repository_text_without_url_normalization() -> Result<()> {
        for path in [
            "Owner/Repo.git",
            "owner/repo?tab=readme",
            "owner/repo#readme",
            "owner/repo%2Ftree",
            "owner/repo ",
        ] {
            let url = format!("https://github.com/{path}");
            assert_eq!(GithubRepo::try_from(url.as_str())?.to_string(), path);
        }
        Ok(())
    }

    #[test]
    fn preserves_parse_error_messages() {
        for (url, expected) in [
            (
                "https://gitlab.com/owner/repo",
                "Expected a GitHub HTTP(S) URL",
            ),
            ("https://github.com/", "Expected a GitHub HTTP(S) URL"),
            ("https://github.com/owner/", "Expected owner/repository"),
            (
                "https://github.com//repo",
                "Expected a repository URL with no subpath",
            ),
            (
                "https://github.com/owner/repo/tree/main",
                "Expected a repository URL with no subpath",
            ),
        ] {
            assert_eq!(
                GithubRepo::try_from(url)
                    .err()
                    .map(|error| error.to_string()),
                Some(expected.to_owned()),
                "{url}"
            );
        }
    }

    #[test]
    fn source_variants_preserve_raw_strings_through_serde() -> Result<()> {
        for (raw, github) in [
            ("https://github.com/Owner/Repo.git", true),
            ("http://github.com/Owner/Repo///", true),
            ("https://github.com/owner/repo?tab=readme", true),
            ("https://github.com/owner/repo#readme", true),
            ("https://github.com/owner/repo%2Ftree", true),
            ("https://github.com/owner/repo ", true),
            ("https://gitlab.com/owner/repo/", false),
            ("https://github.com/owner/repo/tree/main", false),
            ("https://github.com/owner/", false),
            ("https://github.com//repo", false),
            ("https://GitHub.com/owner/repo", false),
            (" https://github.com/owner/repo", false),
            ("git@github.com:owner/repo.git", false),
            ("not a URL", false),
            ("", false),
        ] {
            let encoded = serde_json::to_string(raw)?;
            for source in [
                ToolSource::from(raw),
                ToolSource::from(raw.to_owned()),
                serde_json::from_str::<ToolSource>(&encoded)?,
                serde_saphyr::from_str::<ToolSource>(&encoded)?,
            ] {
                assert_eq!(matches!(source, ToolSource::Github(_)), github, "{raw}");
                assert_eq!(source.as_str(), raw);
                assert_eq!(source.to_string(), raw);
                assert_eq!(serde_json::to_string(&source)?, encoded);
            }
        }
        Ok(())
    }

    #[test]
    fn source_deserialization_matches_plain_string_acceptance() {
        for raw in ["null", "42", "true", "[]", r#"{"Github":"owner/repo"}"#] {
            assert!(serde_json::from_str::<ToolSource>(raw).is_err(), "{raw}");
            assert_eq!(
                serde_saphyr::from_str::<ToolSource>(raw).ok(),
                serde_saphyr::from_str::<String>(raw)
                    .ok()
                    .map(ToolSource::from),
                "{raw}"
            );
        }
    }

    #[test]
    fn repository_owns_its_url() -> Result<()> {
        let repo = {
            let url = String::from("http://github.com/Owner/Repo///");
            GithubRepo::try_from(url.as_str())?
        };
        assert_eq!(repo.to_string(), "Owner/Repo");
        assert_eq!(repo.as_str(), "http://github.com/Owner/Repo///");
        Ok(())
    }

    #[test]
    fn rejects_subpath() {
        assert!(GithubRepo::try_from("https://github.com/owner/repo/tree/main/subdir").is_err());
    }

    #[test]
    fn rejects_gitlab() {
        assert!(GithubRepo::try_from("https://gitlab.com/owner/repo").is_err());
    }

    #[test]
    fn rejects_missing_repo() {
        assert!(GithubRepo::try_from("https://github.com/owner").is_err());
    }

    #[test]
    fn github_url_classification_preserves_supported_forms() {
        for url in [
            "http://github.com/owner/repo",
            "http://github.com/owner/repo/",
            "https://github.com/owner/repo///",
        ] {
            assert_eq!(
                GithubRepo::try_from(url).ok(),
                Some(GithubRepo {
                    owner: "owner".into(),
                    name: "repo".into(),
                    original_url: url.into(),
                })
            );
        }
        for url in [
            "https://github.com/",
            "https://github.com//repo",
            "https://github.com/owner/",
            "https://github.com/owner/repo//tree/main",
            "https://github.com/owner/repo/tree/main",
            "https://GitHub.com/owner/repo",
            " https://github.com/owner/repo",
            "https://github.com.evil/owner/repo",
            "git@github.com:owner/repo.git",
        ] {
            assert!(GithubRepo::try_from(url).is_err(), "{url}");
        }
    }
}
