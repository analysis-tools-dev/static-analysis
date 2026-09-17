//! Read-only GitHub and RDAP access and per-tool check orchestration.

use anyhow::{Context, Result, bail};
use chrono::{DateTime, Utc};
use serde::{Deserialize, de::DeserializeOwned};

use crate::checks::{Check, DomainAge};
use crate::criteria::{
    Contributor, GithubRepo, RepoInfo, ToolEntry, homepage_domain, repository_report,
};
use crate::report::{CheckResult, ToolReport};

pub struct GithubClient {
    client: reqwest::Client,
    token: String,
}

impl GithubClient {
    /// Creates a new client.
    ///
    /// # Errors
    ///
    /// Returns an error if the `reqwest` client cannot be constructed.
    pub fn new(token: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("pr-check-bot/1.0 (analysis-tools-dev)")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to build HTTP client")?;
        Ok(Self { client, token })
    }

    /// Sends an authenticated GET request and deserialises the JSON body.
    ///
    /// # Errors
    ///
    /// Returns an error on network failure or if the response cannot be
    /// deserialised as `T`.
    async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<Option<T>> {
        let resp = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .with_context(|| format!("GET {url} failed"))?;

        let status = resp.status();
        if status == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GET {url} returned {status}: {body}");
        }

        resp.json::<T>()
            .await
            .with_context(|| format!("Failed to deserialise response from {url}"))
            .map(Some)
    }

    /// Fetches repository metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn repo_info(&self, repo: GithubRepo<'_>) -> Result<Option<RepoInfo>> {
        let url = format!("https://api.github.com/repos/{repo}");
        self.get::<RepoInfo>(&url).await
    }

    /// Counts human contributors among the first 100 GitHub contributor accounts.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn contributor_count(&self, repo: GithubRepo<'_>) -> Result<Option<usize>> {
        let url = format!("https://api.github.com/repos/{repo}/contributors?per_page=100&anon=0");
        Ok(self
            .get::<Vec<Contributor>>(&url)
            .await?
            .map(|contributors| contributors.iter().filter(|c| c.counts_as_human()).count()))
    }
}

/// Runs all contributing-criteria checks for one tool.
///
/// # Errors
///
/// Returns an error if date arithmetic fails. Network and authentication failures
/// become unverified checks, while verified criteria failures become failed checks.
pub async fn check_tool(client: &GithubClient, tool: &ToolEntry) -> Result<ToolReport> {
    let source = &tool.source;

    let repo = source
        .as_deref()
        .and_then(|url| GithubRepo::try_from(url).ok());

    if let Some(repo) = repo {
        let repo_result = client.repo_info(repo).await;
        let contributors_result = client.contributor_count(repo).await;

        repository_report(tool, &repo_result, &contributors_result, Utc::now())
    } else {
        let domain = if source.is_none() {
            tool.homepage.as_deref().and_then(homepage_domain)
        } else {
            None
        };
        let age = if let Some(domain) = &domain {
            let registration = fetch_domain_registration(domain).await;
            DomainAge {
                domain,
                registration: &registration,
                now: Utc::now(),
            }
            .check()?
        } else {
            CheckResult::Skip("No supported homepage domain or GitHub source URL".into())
        };
        let note = "No GitHub source URL found. Please verify the contributing criteria manually. \
                    Domain registration dates do not establish when a service launched. \
                    If the service previously operated under another domain, please provide evidence of that history.";

        Ok(ToolReport {
            name: tool.name.clone(),
            source: source.clone(),
            stars: CheckResult::Skip("N/A".into()),
            contributors: CheckResult::Skip("N/A".into()),
            age,
            domain,
            note: Some(note.into()),
        })
    }
}

#[derive(Debug, Deserialize)]
struct RdapDomain {
    #[serde(rename = "ldhName")]
    name: String,
    #[serde(default)]
    events: Vec<RdapEvent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RdapEvent {
    event_action: String,
    event_date: DateTime<Utc>,
}

async fn fetch_domain_registration(domain: &str) -> Result<Option<DateTime<Utc>>> {
    // RDAP requests must never carry the GitHub token. rdap.org redirects to the registry.
    let client = reqwest::Client::builder()
        .user_agent("pr-check-bot/1.0 (analysis-tools-dev)")
        .https_only(true)
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    let response = client
        .get(format!("https://rdap.org/domain/{domain}"))
        .send()
        .await?;
    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    let record = response.error_for_status()?.json::<RdapDomain>().await?;
    if !record.name.eq_ignore_ascii_case(domain) {
        bail!("RDAP returned a different domain");
    }
    Ok(record
        .events
        .into_iter()
        .find(|event| event.event_action == "registration")
        .map(|event| event.event_date))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn unsupported_source_does_not_fall_back_to_homepage_age() -> Result<()> {
        let client = GithubClient::new("unused-test-token".into())?;
        for source in [
            "https://gitlab.com/example/tool",
            "",
            "https://github.com/owner/repo/tree/main",
        ] {
            let tool = ToolEntry {
                name: "Example".into(),
                source: Some(source.into()),
                homepage: Some("https://example.invalid".into()),
            };
            let report = check_tool(&client, &tool).await?;
            assert_eq!(report.status(), "REVIEW");
            assert_eq!(report.source, tool.source);
            assert!(report.domain.is_none());
            assert_eq!(report.stars.message(), "N/A");
            assert_eq!(report.contributors.message(), "N/A");
            assert_eq!(
                report.age.message(),
                "No supported homepage domain or GitHub source URL"
            );
            assert!(report.note.as_deref().is_some_and(|note| {
                note.contains("Please verify the contributing criteria manually")
            }));
        }
        Ok(())
    }

    #[tokio::test]
    async fn missing_source_and_unsupported_homepage_require_review() -> Result<()> {
        let client = GithubClient::new("unused-test-token".into())?;
        for homepage in [None, Some("file:///tmp/tool"), Some("not a URL")] {
            let tool = ToolEntry {
                name: "Example".into(),
                source: None,
                homepage: homepage.map(str::to_owned),
            };
            let report = check_tool(&client, &tool).await?;
            assert_eq!(report.status(), "REVIEW");
            assert!(report.domain.is_none());
            assert!(!report.should_close());
        }
        Ok(())
    }
}
