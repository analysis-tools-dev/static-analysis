//! Shared GitHub repository URL classification.

use anyhow::{Context, Result, ensure};

/// A repository parsed from a GitHub HTTP(S) URL, borrowing its owner and name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GithubRepo<'a> {
    owner: &'a str,
    name: &'a str,
}

impl<'a> TryFrom<&'a str> for GithubRepo<'a> {
    type Error = anyhow::Error;

    /// Parses an HTTP(S) GitHub repository URL, ignoring trailing slashes.
    ///
    /// # Errors
    ///
    /// Returns an error for an unsupported prefix, missing owner or repository,
    /// or a repository path containing a subpath.
    fn try_from(url: &'a str) -> Result<Self> {
        let url = url.trim_end_matches('/');
        let path = url
            .strip_prefix("https://github.com/")
            .or_else(|| url.strip_prefix("http://github.com/"))
            .context("Expected a GitHub HTTP(S) URL")?;
        let (owner, name) = path.split_once('/').context("Expected owner/repository")?;
        ensure!(
            !owner.is_empty() && !name.is_empty() && !name.contains('/'),
            "Expected a repository URL with no subpath"
        );
        Ok(Self { owner, name })
    }
}

impl std::fmt::Display for GithubRepo<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_github_url() -> Result<()> {
        let repo = GithubRepo::try_from("https://github.com/owner/repo")?;
        assert_eq!(
            repo,
            GithubRepo {
                owner: "owner",
                name: "repo"
            }
        );
        assert_eq!(repo.to_string(), "owner/repo");
        Ok(())
    }

    #[test]
    fn parses_trailing_slash() -> Result<()> {
        let repo = GithubRepo::try_from("https://github.com/owner/repo/")?;
        assert_eq!(
            repo,
            GithubRepo {
                owner: "owner",
                name: "repo"
            }
        );
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
                    owner: "owner",
                    name: "repo"
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
