//! PR contribution checker for analysis-tools-dev/static-analysis.
//!
//! Reads new or modified YAML files under `data/tools/` that were introduced
//! by a pull request, fetches metadata from the GitHub API for each tool's
//! source repository, and evaluates each tool against the contributing
//! criteria:
//!
//! - At least 20 stars
//! - More than one contributor
//! - Repository is at least 6 months old
//!
//! Writes the report to `COMMENT_OUTPUT_FILE`, or stdout when unset. The
//! `pr-check` workflow publishes the report and closes verified rejections.
//! Tools without a source URL get an RDAP homepage-domain age check for manual
//! review. This checker only reads metadata; it never modifies PRs.
//!
//! Exit code 2 indicates a verified repository criteria failure warranting
//! closure; exit code 1 indicates an error or a check requiring manual review.
//!
//! Expected environment variables:
//!   `GITHUB_TOKEN`        - a token for reading public repository metadata
//!   `COMMENT_OUTPUT_FILE` - (optional) report output path; defaults to stdout

use anyhow::{Context, Result, bail};
use askama::Template;
use chrono::{DateTime, Months, Utc};
use serde::Deserialize;

use std::env;
use std::path::{Path, PathBuf};

/// A minimal tool entry parsed from `data/tools/<name>.yml`.
/// Only the fields needed for the contributing criteria check are required.
#[derive(Debug, Deserialize)]
struct ToolEntry {
    name: String,
    source: Option<String>,
    homepage: Option<String>,
}

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

const MIN_STARS: u64 = 20;
const MIN_CONTRIBUTORS: usize = 2;
const MIN_AGE_MONTHS: u32 = 6;

// Identifies the report as output from the contribution checker.
const COMMENT_MARKER: &str = "<!-- pr-check-bot -->";

/// The outcome of one criterion check.
#[derive(Debug)]
enum CheckResult {
    Pass(String),
    Fail(String),
    Skip(String),
}

impl CheckResult {
    const fn is_pass(&self) -> bool {
        matches!(self, Self::Pass(_))
    }

    const fn is_fail(&self) -> bool {
        matches!(self, Self::Fail(_))
    }

    const fn symbol(&self) -> &'static str {
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
    /// Domain registration is evidence for review, not proof of service age.
    domain: Option<String>,
    /// Explains checks that require manual review.
    note: Option<String>,
}

impl ToolReport {
    const fn any_fail(&self) -> bool {
        !self.stars.is_pass() || !self.contributors.is_pass() || !self.age.is_pass()
    }

    const fn should_close(&self) -> bool {
        let repository_age_failed = self.age.is_fail() && self.domain.is_none();
        self.stars.is_fail() || self.contributors.is_fail() || repository_age_failed
    }

    const fn status(&self) -> &'static str {
        if self.should_close() {
            "FAIL"
        } else if self.any_fail() {
            "REVIEW"
        } else {
            "PASS"
        }
    }
}

#[derive(Template)]
#[template(path = "comment.md")]
struct CommentTemplate<'a> {
    marker: &'a str,
    reports: &'a [ToolReport],
    any_failures: bool,
    should_close: bool,
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
    async fn get<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Result<Option<T>> {
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
    async fn repo_info(&self, owner: &str, repo: &str) -> Result<Option<RepoInfo>> {
        let url = format!("https://api.github.com/repos/{owner}/{repo}");
        self.get::<RepoInfo>(&url).await
    }

    /// Fetches the contributor list (up to 100, which is enough to confirm
    /// whether there is more than one human contributor).
    ///
    /// # Errors
    ///
    /// Returns an error if the API call fails.
    async fn contributor_count(&self, owner: &str, repo: &str) -> Result<Option<usize>> {
        let url =
            format!("https://api.github.com/repos/{owner}/{repo}/contributors?per_page=100&anon=0");
        let Some(contributors) = self.get::<Vec<Contributor>>(&url).await? else {
            return Ok(None);
        };
        // Exclude bot accounts from the contributor count.
        let human_count = contributors
            .iter()
            .filter(|c| c.account_type != "Bot")
            .count();
        Ok(Some(human_count))
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
fn read_tool(path: &Path) -> Result<ToolEntry> {
    let f = std::fs::File::open(path).with_context(|| format!("Cannot open {}", path.display()))?;
    serde_saphyr::from_reader(f).with_context(|| format!("Cannot parse {}", path.display()))
}

/// Runs all contributing-criteria checks for one tool.
///
/// # Errors
///
/// Returns an error only for unexpected failures (network, auth). Missing
/// criteria produce `CheckResult::Fail` values, not errors.
async fn check_tool(client: &GithubClient, tool: &ToolEntry) -> Result<ToolReport> {
    let source = tool.source.clone();

    let gh_coords = source.as_deref().and_then(parse_github_repo);

    if let Some((owner, repo)) = gh_coords {
        let repo_result = client.repo_info(&owner, &repo).await;
        let contributors_result = client.contributor_count(&owner, &repo).await;

        let stars_check = match &repo_result {
            Ok(Some(info)) => {
                let s = info.stargazers_count;
                if s >= MIN_STARS {
                    CheckResult::Pass(format!("{s} stars"))
                } else {
                    CheckResult::Fail(format!("{s} stars (minimum is {MIN_STARS})"))
                }
            }
            Ok(None) => CheckResult::Skip("repository not found".into()),
            Err(e) => CheckResult::Skip(format!("Could not fetch repo info: {e}")),
        };

        let age_check = match &repo_result {
            Ok(Some(info)) => {
                let now = Utc::now();
                let minimum_created_at = now
                    .checked_sub_months(Months::new(MIN_AGE_MONTHS))
                    .context("Current date cannot be shifted back by six months")?;
                let days = now.signed_duration_since(info.created_at).num_days();

                if info.created_at <= minimum_created_at {
                    CheckResult::Pass(format!("created {days} days ago (at least 6 months)"))
                } else {
                    let eligible_at = info
                        .created_at
                        .checked_add_months(Months::new(MIN_AGE_MONTHS))
                        .with_context(|| {
                            format!(
                                "Repository creation date {} cannot be shifted forward by {MIN_AGE_MONTHS} months",
                                info.created_at
                            )
                        })?;
                    let remaining = eligible_at.signed_duration_since(now).num_days().max(1);
                    CheckResult::Fail(format!(
                        "created {days} days ago, needs {remaining} more days to meet the 6-month minimum"
                    ))
                }
            }
            Ok(None) => CheckResult::Skip("repository not found".into()),
            Err(_) => CheckResult::Skip("Could not determine age (repo info unavailable)".into()),
        };

        let contributors_check = match contributors_result {
            Ok(Some(count)) => {
                if count >= MIN_CONTRIBUTORS {
                    CheckResult::Pass(format!("{count} contributors"))
                } else {
                    CheckResult::Fail(format!(
                        "{count} contributor(s) (minimum is {MIN_CONTRIBUTORS})"
                    ))
                }
            }
            Ok(None) => CheckResult::Skip("repository not found".into()),
            Err(e) => CheckResult::Skip(format!("Could not fetch contributors: {e}")),
        };

        let repo_not_found = matches!(repo_result, Ok(None));
        let note = repo_not_found.then_some(
            "The source URL returned a 404. Please check that the repository exists and is public.",
        );

        Ok(ToolReport {
            name: tool.name.clone(),
            source,
            stars: stars_check,
            contributors: contributors_check,
            age: age_check,
            domain: None,
            note: note.map(str::to_owned),
        })
    } else {
        let domain = if source.is_none() {
            tool.homepage.as_deref().and_then(homepage_domain)
        } else {
            None
        };
        let age = if let Some(domain) = &domain {
            check_domain_age(domain).await
        } else {
            CheckResult::Skip("No supported homepage domain or GitHub source URL".into())
        };
        let note = "No GitHub source URL found. Please verify the contributing criteria manually. \
                    Domain registration dates do not establish when a service launched. \
                    If the service previously operated under another domain, please provide evidence of that history.";

        Ok(ToolReport {
            name: tool.name.clone(),
            source,
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

fn homepage_domain(homepage: &str) -> Option<String> {
    let url = reqwest::Url::parse(homepage).ok()?;
    if !matches!(url.scheme(), "https" | "http") {
        return None;
    }
    let domain = url.domain()?.trim_end_matches('.');
    // Do not fall back to parent domains: their age may belong to a hosting provider.
    Some(domain.strip_prefix("www.").unwrap_or(domain).to_owned())
}

async fn check_domain_age(domain: &str) -> CheckResult {
    match fetch_domain_registration(domain).await {
        Ok(Some(registered)) => domain_age_result(domain, registered, Utc::now()),
        Ok(None) => {
            CheckResult::Skip("Domain registration date unavailable; manual review required".into())
        }
        Err(error) => CheckResult::Skip(format!("Could not check domain registration: {error}")),
    }
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

fn domain_age_result(domain: &str, registered: DateTime<Utc>, now: DateTime<Utc>) -> CheckResult {
    let Some(eligible) = registered.checked_add_months(Months::new(MIN_AGE_MONTHS)) else {
        return CheckResult::Skip("Invalid domain registration date".into());
    };
    if registered > now {
        return CheckResult::Skip(
            "Domain registration date is in the future; manual review required".into(),
        );
    }
    let message = format!(
        "The homepage domain `{domain}` was registered on {} and reaches the six-month minimum on {}.",
        registered.format("%B %-d, %Y"),
        eligible.format("%B %-d, %Y")
    );
    if now < eligible {
        CheckResult::Fail(message)
    } else {
        CheckResult::Pass(format!(
            "Domain registered on {} (at least six months ago). Service age still requires manual review.",
            registered.format("%B %-d, %Y")
        ))
    }
}

/// Renders all tool reports into a Markdown comment body.
///
/// # Errors
///
/// Returns an error if the template fails to render.
fn render_comment(reports: &[ToolReport]) -> Result<String> {
    let any_failures = reports.iter().any(ToolReport::any_fail);
    CommentTemplate {
        marker: COMMENT_MARKER,
        reports,
        any_failures,
        should_close: reports.iter().any(ToolReport::should_close),
    }
    .render()
    .context("Failed to render comment template")
}

fn report_exit_code(reports: &[ToolReport]) -> i32 {
    if reports.iter().any(ToolReport::should_close) {
        2
    } else {
        i32::from(reports.iter().any(ToolReport::any_fail))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;

    // Remaining CLI arguments are the paths to check.
    // Usage: pr-check data/tools/foo.yml data/tools/bar.yml
    let pico = pico_args::Arguments::from_env();
    let tool_paths: Vec<PathBuf> = pico
        .finish()
        .into_iter()
        .map(PathBuf::from)
        .filter(|p| {
            p.starts_with("data/tools")
                && matches!(
                    p.extension().and_then(|extension| extension.to_str()),
                    Some("yml" | "yaml")
                )
        })
        .collect();

    let client = GithubClient::new(token)?;

    let mut reports = Vec::new();
    for path in &tool_paths {
        let tool = read_tool(path).with_context(|| format!("Failed to read {}", path.display()))?;
        eprintln!("Checking '{}'...", tool.name);
        let report = check_tool(&client, &tool).await?;
        reports.push(report);
    }

    let comment_body = render_comment(&reports)?;

    if let Some(output_file) = env::var("COMMENT_OUTPUT_FILE")
        .ok()
        .filter(|s| !s.is_empty())
    {
        if let Some(parent) = std::path::Path::new(&output_file).parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory for {output_file}"))?;
        }
        std::fs::write(&output_file, &comment_body)
            .with_context(|| format!("Failed to write comment to {output_file}"))?;
        eprintln!("Comment written to {output_file}");
    } else {
        println!("{comment_body}");
    }

    let exit_code = report_exit_code(&reports);
    if exit_code != 0 {
        eprintln!(
            "One or more tools failed or require manual review of the contributing criteria."
        );
        std::process::exit(exit_code);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_catalog() -> Result<()> {
        let tools = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/tools");
        let mut count = 0;
        for entry in std::fs::read_dir(tools)? {
            let path = entry?.path();
            if path.extension().is_some_and(|ext| ext == "yml") {
                let tool = read_tool(&path)?;
                assert!(!tool.name.is_empty(), "{}", path.display());
                count += 1;
            }
        }
        assert!(count > 0);
        Ok(())
    }

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

    fn passing_report() -> ToolReport {
        ToolReport {
            name: "Example".into(),
            source: Some("https://github.com/example/tool".into()),
            stars: CheckResult::Pass("20 stars".into()),
            contributors: CheckResult::Pass("2 contributors".into()),
            age: CheckResult::Pass("at least 6 months".into()),
            domain: None,
            note: None,
        }
    }

    #[test]
    fn passing_tools_do_not_close_pr() -> Result<()> {
        let reports = [passing_report()];
        assert_eq!(report_exit_code(&reports), 0);
        let comment = render_comment(&reports)?;
        assert!(comment.contains("All tool eligibility criteria passed"));
        assert!(!comment.contains("closing this pull request"));
        assert_eq!(report_exit_code(&[]), 0);
        Ok(())
    }

    #[test]
    fn each_verified_failure_closes_pr_and_invites_resubmission() -> Result<()> {
        for criterion in 0..3 {
            let mut report = passing_report();
            let check = match criterion {
                0 => &mut report.stars,
                1 => &mut report.contributors,
                _ => &mut report.age,
            };
            *check = CheckResult::Fail("below minimum".into());
            assert_eq!(report.status(), "FAIL");
            let reports = [passing_report(), report];
            assert_eq!(report_exit_code(&reports), 2);
            let comment = render_comment(&reports)?;
            assert!(comment.contains("closing this pull request"));
            assert!(comment.contains("submit a new pull request once all criteria are met"));
        }
        Ok(())
    }

    #[test]
    fn unverified_checks_require_review_not_closure() -> Result<()> {
        for reason in ["N/A", "repository not found", "GitHub API unavailable"] {
            let mut report = passing_report();
            report.stars = CheckResult::Skip(reason.into());
            report.contributors = CheckResult::Skip(reason.into());
            report.age = CheckResult::Skip(reason.into());
            assert_eq!(report.status(), "REVIEW");
            let reports = [report];
            assert_eq!(report_exit_code(&reports), 1);
            let comment = render_comment(&reports)?;
            assert!(comment.contains("needs manual review"));
            assert!(!comment.contains("closing this pull request"));
        }
        Ok(())
    }

    #[test]
    fn verified_failure_still_closes_when_another_check_is_unverified() {
        let mut report = passing_report();
        report.stars = CheckResult::Skip("GitHub API unavailable".into());
        report.contributors = CheckResult::Fail("1 contributor".into());
        assert_eq!(report_exit_code(&[report]), 2);
    }

    #[test]
    fn homepage_domains_are_not_reduced_to_hosting_providers() {
        assert_eq!(
            homepage_domain("https://www.battletest.dev/path"),
            Some("battletest.dev".into())
        );
        assert_eq!(
            homepage_domain("https://tool.github.io"),
            Some("tool.github.io".into())
        );
        assert_eq!(
            homepage_domain("https://app.example.co.uk"),
            Some("app.example.co.uk".into())
        );
        for url in [
            "not a URL",
            "file:///tmp/tool",
            "https://127.0.0.1",
            "https://[::1]",
        ] {
            assert!(homepage_domain(url).is_none());
        }
    }

    #[test]
    fn domain_age_uses_six_calendar_months() -> Result<()> {
        let registered = "2026-05-01T20:44:07Z".parse::<DateTime<Utc>>()?;
        let before = "2026-11-01T20:44:06Z".parse::<DateTime<Utc>>()?;
        let boundary = "2026-11-01T20:44:07Z".parse::<DateTime<Utc>>()?;
        let result = domain_age_result("battletest.dev", registered, before);
        assert!(result.is_fail());
        assert!(result.message().contains("registered on May 1, 2026"));
        assert!(result.message().contains("minimum on November 1, 2026"));
        assert!(domain_age_result("battletest.dev", registered, boundary).is_pass());
        assert!(matches!(
            domain_age_result("battletest.dev", boundary, registered),
            CheckResult::Skip(_)
        ));
        Ok(())
    }

    #[test]
    fn domain_checks_require_review_without_closing() -> Result<()> {
        for age in [
            CheckResult::Fail("Domain younger than six months".into()),
            CheckResult::Pass("Domain older than six months".into()),
            CheckResult::Skip("RDAP unavailable".into()),
        ] {
            let mut report = passing_report();
            report.source = None;
            report.domain = Some("battletest.dev".into());
            report.stars = CheckResult::Skip("N/A".into());
            report.contributors = CheckResult::Skip("N/A".into());
            report.age = age;
            assert!(!report.should_close());
            assert_eq!(report.status(), "REVIEW");
            let reports = [report];
            assert_eq!(report_exit_code(&reports), 1);
            let comment = render_comment(&reports)?;
            assert!(comment.contains("Homepage domain age"));
            assert!(comment.contains("https://rdap.org/domain/battletest.dev"));
            assert!(!comment.contains("closing this pull request"));
            assert!(comment.contains("needs manual review"));
        }
        Ok(())
    }

    #[test]
    fn render_comment_no_files() -> Result<()> {
        let comment = render_comment(&[])?;
        assert!(comment.contains("No new tool files detected"));
        Ok(())
    }

    #[test]
    fn render_comment_contains_marker() -> Result<()> {
        let comment = render_comment(&[])?;
        assert!(comment.contains(COMMENT_MARKER));
        Ok(())
    }
}
