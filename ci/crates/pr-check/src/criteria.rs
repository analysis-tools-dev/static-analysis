//! Contribution criteria and URL classification, independent of network access.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::checks::{Check, Contributors, RepositoryAge, Stars};
use crate::input::ToolEntry;
use crate::report::ToolReport;

/// Response from `GET /repos/{owner}/{repo}`.
#[derive(Debug, Deserialize)]
pub struct RepoInfo {
    pub stargazers_count: u64,
    pub created_at: DateTime<Utc>,
}

/// One item from `GET /repos/{owner}/{repo}/contributors`.
#[derive(Debug, Deserialize)]
pub struct Contributor {
    login: String,
    #[serde(rename = "type")]
    account_type: String,
}

impl Contributor {
    pub fn counts_as_human(&self) -> bool {
        let login = self.login.to_ascii_lowercase();
        self.account_type.eq_ignore_ascii_case("User")
            && !login.ends_with("[bot]")
            && !AUTOMATION_LOGINS.contains(&login.as_str())
    }
}

// Some automation accounts are reported as ordinary users by GitHub.
// Use exact logins rather than broad patterns that could exclude human contributors.
const AUTOMATION_LOGINS: &[&str] = &["claude", "dependabot", "renovate-bot"];

/// Evaluate fetched metadata without performing I/O; unavailable checks require review.
pub fn repository_report(
    tool: &ToolEntry,
    repo_result: &Result<Option<RepoInfo>>,
    contributors_result: &Result<Option<usize>>,
    now: DateTime<Utc>,
) -> Result<ToolReport> {
    let stars = Stars {
        repository: repo_result,
    }
    .check()?;
    let age = RepositoryAge {
        repository: repo_result,
        now,
    }
    .check()?;
    let contributors = Contributors {
        count: contributors_result,
    }
    .check()?;

    let repo_not_found = matches!(repo_result, Ok(None));
    let note = repo_not_found.then_some(
        "The source URL returned a 404. Please check that the repository exists and is public.",
    );

    Ok(ToolReport {
        name: tool.name.clone(),
        source: tool.source.clone(),
        stars,
        contributors,
        age,
        domain: None,
        note: note.map(str::to_owned),
    })
}

pub fn homepage_domain(homepage: &str) -> Option<String> {
    let url = reqwest::Url::parse(homepage).ok()?;
    if !matches!(url.scheme(), "https" | "http") {
        return None;
    }
    let domain = url.domain()?.trim_end_matches('.');
    // Do not fall back to parent domains: their age may belong to a hosting provider.
    Some(domain.strip_prefix("www.").unwrap_or(domain).to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excludes_non_user_account_types() {
        for account_type in ["Bot", "bot", "Organization", "unknown"] {
            let contributor = Contributor {
                login: "otherwise-ordinary-name".into(),
                account_type: account_type.into(),
            };
            assert!(!contributor.counts_as_human(), "{account_type}");
        }
    }

    #[test]
    fn keeps_humans_with_similar_names() {
        for login in [
            "alice",
            "claude-smith",
            "dependabot-maintainer",
            "robotics-researcher",
            "human-bot",
        ] {
            let contributor = Contributor {
                login: login.into(),
                account_type: "User".into(),
            };
            assert!(contributor.counts_as_human(), "{login}");
        }
    }

    #[test]
    fn human_and_automation_do_not_meet_contributor_minimum() -> Result<()> {
        let contributors: Vec<Contributor> = serde_saphyr::from_str(
            "- {login: alice, type: User}\n- {login: claude, type: User}\n- {login: 'dependabot[bot]', type: Bot}\n- {login: renovate-bot, type: User}\n- {login: bob, type: User}",
        )?;
        let count =
            |accounts: &[Contributor]| accounts.iter().filter(|c| c.counts_as_human()).count();
        assert_eq!(count(&contributors[..4]), 1);
        assert!(count(&contributors[..4]) < 2);
        assert_eq!(count(&contributors), 2);
        Ok(())
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
    fn excludes_automation_even_when_github_reports_a_user() {
        for login in [
            "claude",
            "Claude",
            "dependabot",
            "Dependabot",
            "renovate-bot",
            "RENOVATE-BOT",
            "dependabot[bot]",
            "github-actions[bot]",
            "copilot[bot]",
            "coderabbitai[bot]",
            "some-new-app[BOT]",
        ] {
            let contributor = Contributor {
                login: login.into(),
                account_type: "User".into(),
            };
            assert!(!contributor.counts_as_human(), "{login}");
        }
    }

    fn example_tool() -> ToolEntry {
        ToolEntry {
            name: "Example".into(),
            source: Some("https://github.com/example/tool".into()),
            homepage: None,
        }
    }

    #[test]
    fn repository_thresholds_and_calendar_age_boundary() -> Result<()> {
        let created_at = "2026-03-01T12:00:00Z".parse::<DateTime<Utc>>()?;
        let boundary = "2026-09-01T12:00:00Z".parse::<DateTime<Utc>>()?;
        for stars in [19, 20, 21] {
            for contributors in [0, 1, 2, 3] {
                for seconds in [-1, 0, 1] {
                    let report = repository_report(
                        &example_tool(),
                        &Ok(Some(RepoInfo {
                            stargazers_count: stars,
                            created_at,
                        })),
                        &Ok(Some(contributors)),
                        boundary + chrono::Duration::seconds(seconds),
                    )?;
                    assert_eq!(report.stars.is_pass(), stars >= 20);
                    assert_eq!(report.contributors.is_pass(), contributors >= 2);
                    assert_eq!(report.age.is_pass(), seconds >= 0);
                    assert_eq!(
                        report.should_close(),
                        stars < 20 || contributors < 2 || seconds < 0
                    );
                    if seconds == -1 {
                        assert!(report.age.message().contains("needs 1 more days"));
                    }
                }
            }
        }
        Ok(())
    }

    #[test]
    fn repository_age_preserves_month_end_cutoff() -> Result<()> {
        // The repository rule subtracts six calendar months from the check date.
        let now = "2026-08-31T12:00:00Z".parse::<DateTime<Utc>>()?;
        for (created, passes) in [
            ("2026-02-28T12:00:00Z", true),
            ("2026-02-28T12:00:01Z", false),
        ] {
            let report = repository_report(
                &example_tool(),
                &Ok(Some(RepoInfo {
                    stargazers_count: 20,
                    created_at: created.parse()?,
                })),
                &Ok(Some(2)),
                now,
            )?;
            assert_eq!(report.age.is_pass(), passes);
        }
        Ok(())
    }

    #[test]
    fn missing_or_unavailable_metadata_is_not_a_verified_failure() -> Result<()> {
        let now = "2026-09-01T12:00:00Z".parse::<DateTime<Utc>>()?;
        let missing = repository_report(&example_tool(), &Ok(None), &Ok(None), now)?;
        assert_eq!(missing.status(), "REVIEW");
        assert_eq!(missing.stars.message(), "repository not found");
        assert_eq!(missing.age.message(), "repository not found");
        assert_eq!(missing.contributors.message(), "repository not found");
        assert!(
            missing
                .note
                .as_deref()
                .is_some_and(|note| note.contains("404"))
        );

        let unavailable = repository_report(
            &example_tool(),
            &Err(anyhow::anyhow!("rate limited")),
            &Err(anyhow::anyhow!("connection failed")),
            now,
        )?;
        assert_eq!(unavailable.status(), "REVIEW");
        assert_eq!(
            unavailable.stars.message(),
            "Could not fetch repo info: rate limited"
        );
        assert_eq!(
            unavailable.age.message(),
            "Could not determine age (repo info unavailable)"
        );
        assert_eq!(
            unavailable.contributors.message(),
            "Could not fetch contributors: connection failed"
        );
        assert!(unavailable.note.is_none());

        let verified = repository_report(
            &example_tool(),
            &Err(anyhow::anyhow!("rate limited")),
            &Ok(Some(1)),
            now,
        )?;
        assert_eq!(verified.status(), "FAIL");
        Ok(())
    }

    #[test]
    fn unavailable_contributors_do_not_hide_verified_repository_failures() -> Result<()> {
        let now = "2026-09-01T12:00:00Z".parse::<DateTime<Utc>>()?;
        for (stars, created, expected) in [
            (20, "2020-01-01T00:00:00Z", "REVIEW"),
            (19, "2020-01-01T00:00:00Z", "FAIL"),
            (20, "2026-08-01T00:00:00Z", "FAIL"),
        ] {
            let report = repository_report(
                &example_tool(),
                &Ok(Some(RepoInfo {
                    stargazers_count: stars,
                    created_at: created.parse()?,
                })),
                &Err(anyhow::anyhow!("API unavailable")),
                now,
            )?;
            assert_eq!(report.status(), expected);
        }
        Ok(())
    }
}
