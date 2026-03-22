//! PR contribution checker for analysis-tools-dev/static-analysis.
//!
//! Reads new or modified YAML files under `data/tools/` that were introduced
//! by a pull request, fetches metadata from the GitHub API for each tool's
//! source repository, and evaluates each tool against the contributing
//! criteria:
//!
//! - More than 20 stars
//! - More than one contributor
//! - Repository is at least 3 months old
//!
//! The results are posted as a single comment on the PR (updating an existing
//! bot comment if one already exists). The process exits with a non-zero status
//! code when any hard criterion is not met, causing CI to fail.
//!
//! Expected environment variables:
//!   GITHUB_TOKEN   - a token with `pull-requests: write` permission
//!   GITHUB_REPOSITORY - owner/repo, e.g. "analysis-tools-dev/static-analysis"
//!   PR_NUMBER      - the pull request number

use anyhow::{Context, Result, bail};
use askama::Template;
use chrono::{DateTime, Duration, Utc};
use render::types::ParsedEntry;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

/// Response from `GET /repos/{owner}/{repo}`.
#[derive(Debug, Deserialize)]
struct RepoInfo {
    stargazers_count: u64,
    created_at: DateTime<Utc>,
}

/// One item from `GET /repos/{owner}/{repo}/contributors`.
#[derive(Debug, Deserialize)]
struct Contributor {
    #[serde(rename = "type")]
    account_type: String,
}

/// One PR comment from `GET /repos/{owner}/{repo}/issues/{pr}/comments`.
#[derive(Debug, Deserialize)]
struct IssueComment {
    id: u64,
    body: String,
}

const MIN_STARS: u64 = 20;
const MIN_CONTRIBUTORS: usize = 2;
const MIN_AGE_DAYS: i64 = 90;

// Marker text embedded in every comment we post so we can find and update it.
const COMMENT_MARKER: &str = "<!-- pr-check-bot -->";

/// The outcome of one criterion check.
#[derive(Debug)]
enum CheckResult {
    Pass(String),
    Fail(String),
    Skip(String),
}

impl CheckResult {
    fn is_fail(&self) -> bool {
        matches!(self, Self::Fail(_))
    }

    fn symbol(&self) -> &'static str {
        match self {
            Self::Pass(_) => "pass",
            Self::Fail(_) => "fail",
            Self::Skip(_) => "skip",
        }
    }

    fn message(&self) -> &str {
        match self {
            Self::Pass(m) | Self::Fail(m) | Self::Skip(m) => m,
        }
    }
}

/// All checks for a single tool.
#[derive(Debug)]
struct ToolReport {
    name: String,
    source: Option<String>,
    stars: CheckResult,
    contributors: CheckResult,
    age: CheckResult,
    /// Non-GitHub source repositories cannot be checked automatically.
    note: Option<String>,
}

impl ToolReport {
    fn any_fail(&self) -> bool {
        self.stars.is_fail() || self.contributors.is_fail() || self.age.is_fail()
    }

    fn status(&self) -> &'static str {
        if self.any_fail() { "FAIL" } else { "PASS" }
    }
}

#[derive(Template)]
#[template(path = "comment.md")]
struct CommentTemplate<'a> {
    marker: &'a str,
    reports: &'a [ToolReport],
    any_failures: bool,
}

struct GithubClient {
    client: reqwest::Client,
    token: String,
}

impl GithubClient {
    /// Creates a new client.
    ///
    /// # Errors
    ///
    /// Returns an error if the `reqwest` client cannot be constructed.
    fn new(token: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("pr-check-bot/1.0 (analysis-tools-dev)")
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
    async fn get<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<T> {
        let resp = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .with_context(|| format!("GET {url} failed"))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GET {url} returned {status}: {body}");
        }

        resp.json::<T>()
            .await
            .with_context(|| format!("Failed to deserialise response from {url}"))
    }

    /// Fetches repository metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn repo_info(&self, owner: &str, repo: &str) -> Result<RepoInfo> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}");
        self.get::<RepoInfo>(&url).await
    }

    /// Fetches the contributor list (up to 100, which is enough to confirm
    /// whether there is more than one human contributor).
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn contributor_count(&self, owner: &str, repo: &str) -> Result<usize> {
        let url =
            format!("https://api.github.com/repos/{owner}/{repo}/contributors?per_page=100&anon=0");
        let contributors = self.get::<Vec<Contributor>>(&url).await?;
        // Exclude bot accounts from the contributor count.
        let human_count = contributors
            .iter()
            .filter(|c| c.account_type != "Bot")
            .count();
        Ok(human_count)
    }

    /// Lists all comments on a PR/issue.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn list_pr_comments(&self, repo: &str, pr: u64) -> Result<Vec<IssueComment>> {
        let url = format!("https://api.github.com/repos/{repo}/issues/{pr}/comments?per_page=100");
        self.get::<Vec<IssueComment>>(&url).await
    }

    /// Creates a new PR comment.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn create_pr_comment(&self, repo: &str, pr: u64, body: &str) -> Result<()> {
        let url = format!("https://api.github.com/repos/{repo}/issues/{pr}/comments");
        let mut payload = HashMap::new();
        payload.insert("body", body);

        let resp = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&payload)
            .send()
            .await
            .with_context(|| format!("POST {url} failed"))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("POST {url} returned {status}: {body}");
        }
        Ok(())
    }

    /// Updates an existing PR comment.
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn update_pr_comment(&self, repo: &str, comment_id: u64, body: &str) -> Result<()> {
        let url = format!("https://api.github.com/repos/{repo}/issues/comments/{comment_id}");
        let mut payload = HashMap::new();
        payload.insert("body", body);

        let resp = self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .json(&payload)
            .send()
            .await
            .with_context(|| format!("PATCH {url} failed"))?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("PATCH {url} returned {status}: {body}");
        }
        Ok(())
    }
}

/// Parses `owner` and `repo` out of a GitHub URL like
/// `https://github.com/owner/repo` or `https://github.com/owner/repo/`.
/// Returns `None` for non-GitHub URLs or malformed paths.
fn parse_github_repo(url: &str) -> Option<(String, String)> {
    let url = url.trim_end_matches('/');
    let without_scheme = url
        .strip_prefix("https://github.com/")
        .or_else(|| url.strip_prefix("http://github.com/"))?;

    let parts: Vec<&str> = without_scheme.splitn(3, '/').collect();
    if parts.len() < 2 || parts[0].is_empty() || parts[1].is_empty() {
        return None;
    }
    // Reject sub-paths inside a repo (e.g. /tree/main/…).
    if parts.len() == 3 && !parts[2].is_empty() {
        return None;
    }
    Some((parts[0].to_owned(), parts[1].to_owned()))
}

/// Reads and deserialises a single tool YAML file.
///
/// # Errors
///
/// Returns an error if the file cannot be read or parsed.
fn read_tool(path: &Path) -> Result<ParsedEntry> {
    let f = std::fs::File::open(path).with_context(|| format!("Cannot open {}", path.display()))?;
    serde_yaml::from_reader(f).with_context(|| format!("Cannot parse {}", path.display()))
}

/// Runs all contributing-criteria checks for one tool.
///
/// # Errors
///
/// Returns an error only for unexpected failures (network, auth). Missing
/// criteria produce `CheckResult::Fail` values, not errors.
async fn check_tool(client: &GithubClient, tool: &ParsedEntry) -> Result<ToolReport> {
    let source = tool.source.clone();

    let gh_coords = source.as_deref().and_then(parse_github_repo);

    if let Some((owner, repo)) = gh_coords {
        let repo_result = client.repo_info(&owner, &repo).await;
        let contributors_result = client.contributor_count(&owner, &repo).await;

        let stars_check = match &repo_result {
            Ok(info) => {
                let s = info.stargazers_count;
                if s >= MIN_STARS {
                    CheckResult::Pass(format!("{s} stars"))
                } else {
                    CheckResult::Fail(format!("{s} stars (minimum is {MIN_STARS})"))
                }
            }
            Err(e) => CheckResult::Fail(format!("Could not fetch repo info: {e}")),
        };

        let age_check = match &repo_result {
            Ok(info) => {
                let age = Utc::now().signed_duration_since(info.created_at);
                let days = age.num_days();
                let months = days / 30;
                if age >= Duration::days(MIN_AGE_DAYS) {
                    CheckResult::Pass(format!("created {days} days ago (~{months} months)"))
                } else {
                    let remaining = MIN_AGE_DAYS - days;
                    CheckResult::Fail(format!(
                        "created {days} days ago, needs {remaining} more days to meet the 3-month minimum"
                    ))
                }
            }
            Err(_) => CheckResult::Skip("Could not determine age (repo info unavailable)".into()),
        };

        let contributors_check = match contributors_result {
            Ok(count) => {
                if count >= MIN_CONTRIBUTORS {
                    CheckResult::Pass(format!("{count} contributors"))
                } else {
                    CheckResult::Fail(format!(
                        "{count} contributor(s) (minimum is {MIN_CONTRIBUTORS})"
                    ))
                }
            }
            Err(e) => CheckResult::Fail(format!("Could not fetch contributors: {e}")),
        };

        Ok(ToolReport {
            name: tool.name.to_string(),
            source,
            stars: stars_check,
            contributors: contributors_check,
            age: age_check,
            note: None,
        })
    } else {
        // No source or non-GitHub source. This is fine for proprietary or
        // hosted tools. Skip automated checks and leave a note for manual review.
        let note = "No GitHub source URL found. Automated checks for stars, contributor count, \
                    and age are not possible. Please verify the contributing criteria manually.";

        Ok(ToolReport {
            name: tool.name.to_string(),
            source,
            stars: CheckResult::Skip("N/A".into()),
            contributors: CheckResult::Skip("N/A".into()),
            age: CheckResult::Skip("N/A".into()),
            note: Some(note.into()),
        })
    }
}

/// Renders all tool reports into a Markdown comment body.
///
/// # Errors
///
/// Returns an error if the template fails to render.
fn render_comment(reports: &[ToolReport]) -> Result<String> {
    let any_failures = reports.iter().any(|r| r.any_fail());
    CommentTemplate {
        marker: COMMENT_MARKER,
        reports,
        any_failures,
    }
    .render()
    .context("Failed to render comment template")
}

/// Posts or updates the bot comment on the PR.
///
/// # Errors
///
/// Returns an error if the GitHub API calls fail.
async fn upsert_comment(client: &GithubClient, repo: &str, pr: u64, body: &str) -> Result<()> {
    let comments = client.list_pr_comments(repo, pr).await?;

    let existing = comments.iter().find(|c| c.body.contains(COMMENT_MARKER));

    match existing {
        Some(c) => client.update_pr_comment(repo, c.id, body).await,
        None => client.create_pr_comment(repo, pr, body).await,
    }
}

/// Parses a PR number from a string.
///
/// # Errors
///
/// Returns an error if the string is not a valid integer.
fn parse_pr_number(s: &str) -> Result<u64> {
    s.trim()
        .parse::<u64>()
        .with_context(|| format!("Invalid PR number: {s}"))
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;
    let gh_repo = env::var("GITHUB_REPOSITORY").context("GITHUB_REPOSITORY not set")?;
    let pr_number_str = env::var("PR_NUMBER").context("PR_NUMBER not set")?;
    let pr_number = parse_pr_number(&pr_number_str)?;

    // Remaining CLI arguments are the paths to check.
    // Usage: pr-check data/tools/foo.yml data/tools/bar.yml
    let pico = pico_args::Arguments::from_env();
    let tool_paths: Vec<PathBuf> = pico.finish().into_iter().map(PathBuf::from).collect();

    let tool_paths: Vec<PathBuf> = tool_paths
        .into_iter()
        .filter(|p| {
            p.starts_with("data/tools")
                && matches!(
                    p.extension().and_then(|e| e.to_str()),
                    Some("yml") | Some("yaml")
                )
        })
        .collect();

    let client = GithubClient::new(token)?;

    let mut reports = Vec::new();
    for path in &tool_paths {
        let tool = read_tool(path).with_context(|| format!("Failed to read {}", path.display()))?;
        eprintln!("Checking '{}'...", &tool.name);
        let report = check_tool(&client, &tool).await?;
        reports.push(report);
    }

    let comment_body = render_comment(&reports)?;

    upsert_comment(&client, &gh_repo, pr_number, &comment_body).await?;

    let any_failures = reports.iter().any(|r| r.any_fail());
    if any_failures {
        eprintln!("One or more tools failed the contributing criteria check.");
        std::process::exit(1);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_github_url() {
        let result = parse_github_repo("https://github.com/owner/repo");
        assert_eq!(result, Some(("owner".into(), "repo".into())));
    }

    #[test]
    fn parses_trailing_slash() {
        let result = parse_github_repo("https://github.com/owner/repo/");
        assert_eq!(result, Some(("owner".into(), "repo".into())));
    }

    #[test]
    fn rejects_subpath() {
        let result = parse_github_repo("https://github.com/owner/repo/tree/main/subdir");
        assert!(result.is_none());
    }

    #[test]
    fn rejects_gitlab() {
        let result = parse_github_repo("https://gitlab.com/owner/repo");
        assert!(result.is_none());
    }

    #[test]
    fn rejects_missing_repo() {
        let result = parse_github_repo("https://github.com/owner");
        assert!(result.is_none());
    }

    #[test]
    fn render_comment_no_files() {
        let comment = render_comment(&[]).unwrap();
        assert!(comment.contains("No new tool files detected"));
    }

    #[test]
    fn render_comment_contains_marker() {
        let comment = render_comment(&[]).unwrap();
        assert!(comment.contains(COMMENT_MARKER));
    }
}
