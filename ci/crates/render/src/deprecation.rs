use anyhow::{Context, Result};
use chrono::{DateTime, Local, NaiveDate, Utc};
use github_repo::{GithubRepo, ToolSource};
use serde::Deserialize;

use crate::types::Entry;

#[derive(Deserialize)]
struct CommitResponse {
    commit: Commit,
}

#[derive(Deserialize)]
struct Commit {
    author: CommitAuthor,
}

#[derive(Deserialize)]
struct CommitAuthor {
    date: DateTime<Utc>,
}

struct GithubClient {
    client: reqwest::Client,
    token: String,
}

impl GithubClient {
    fn new(token: &str) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(concat!("analysis-tools-render/", env!("CARGO_PKG_VERSION")))
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .context("Failed to build GitHub HTTP client")?;
        Ok(Self {
            client,
            token: token.to_owned(),
        })
    }

    async fn latest_commit_date(&self, repo: &GithubRepo) -> Result<Option<DateTime<Utc>>> {
        let url = format!("https://api.github.com/repos/{repo}/commits?per_page=1");
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .with_context(|| format!("Failed to fetch commits for {repo}"))?;

        if matches!(
            response.status(),
            reqwest::StatusCode::NOT_FOUND | reqwest::StatusCode::CONFLICT
        ) {
            return Ok(None);
        }

        let commits = response
            .error_for_status()
            .with_context(|| format!("GitHub rejected the commits request for {repo}"))?
            .json::<Vec<CommitResponse>>()
            .await
            .with_context(|| format!("Invalid commits response for {repo}"))?;

        Ok(commits
            .into_iter()
            .next()
            .map(|commit| commit.commit.author.date))
    }
}

fn deprecation_marker(today: NaiveDate, last_commit: DateTime<Utc>) -> Option<bool> {
    // Preserve the existing calendar-date policy: local today versus the UTC
    // author's date, not elapsed hours or the commit's committer date.
    (today
        .signed_duration_since(last_commit.date_naive())
        .num_days()
        > 365)
        .then_some(true)
}

/// Refreshes deprecation markers using each GitHub repository's latest commit.
///
/// Unavailable repositories and failed requests leave their markers unchanged.
/// Request failures are reported to stderr without stopping subsequent checks.
///
/// # Errors
///
/// Returns an error when the HTTP client cannot be created.
pub async fn check_deprecated(token: &str, entries: &mut [Entry]) -> Result<()> {
    let client = GithubClient::new(token)?;

    for entry in entries {
        let Some(ToolSource::Github(repo)) = &entry.source else {
            continue;
        };
        let last_commit = match client.latest_commit_date(repo).await {
            Ok(Some(date)) => date,
            Ok(None) => continue,
            Err(error) => {
                eprintln!("Could not check {repo} for deprecation: {error:#}");
                continue;
            }
        };

        entry.deprecated = deprecation_marker(Local::now().date_naive(), last_commit);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_github_author_date_not_committer_date() -> Result<()> {
        let response: Vec<CommitResponse> = serde_json::from_str(
            r#"[{"commit":{"author":{"date":"2026-08-01T12:34:56Z"},"committer":{"date":"2026-09-01T00:00:00Z"}}}]"#,
        )?;
        let date = response
            .into_iter()
            .next()
            .map(|commit| commit.commit.author.date);
        assert_eq!(
            date.map(|value| value.to_rfc3339()),
            Some("2026-08-01T12:34:56+00:00".into())
        );
        Ok(())
    }

    #[test]
    fn deprecation_uses_calendar_dates_and_a_strict_365_day_cutoff() -> Result<()> {
        let today = "2026-09-17".parse()?;
        for (timestamp, expected) in [
            ("2025-09-16T23:59:59Z", Some(true)),
            ("2025-09-17T00:00:00Z", None),
            ("2025-09-18T00:00:00Z", None),
            ("2026-09-17T23:59:59Z", None),
            ("2026-09-18T00:00:00Z", None),
            ("2025-09-17T00:30:00+01:00", Some(true)),
        ] {
            assert_eq!(
                deprecation_marker(today, timestamp.parse()?),
                expected,
                "{timestamp}"
            );
        }
        assert_eq!(
            deprecation_marker("2024-03-01".parse()?, "2023-03-01T00:00:00Z".parse()?),
            Some(true)
        );
        Ok(())
    }
}
