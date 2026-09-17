//! Report status, Markdown rendering, and workflow exit-code contract.

use askama::Template;

// Identifies the report as output from the contribution checker.
const COMMENT_MARKER: &str = "<!-- pr-check-bot -->";

/// The outcome of one criterion check.
#[derive(Debug)]
pub enum CheckResult {
    Pass(String),
    Fail(String),
    Skip(String),
}

impl CheckResult {
    pub const fn is_pass(&self) -> bool {
        matches!(self, Self::Pass(_))
    }

    pub const fn is_fail(&self) -> bool {
        matches!(self, Self::Fail(_))
    }

    pub const fn symbol(&self) -> &'static str {
        match self {
            Self::Pass(_) => "pass",
            Self::Fail(_) => "fail",
            Self::Skip(_) => "skip",
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Pass(m) | Self::Fail(m) | Self::Skip(m) => m,
        }
    }
}

/// All checks for a single tool.
#[derive(Debug)]
pub struct ToolReport {
    pub name: String,
    pub source: Option<String>,
    pub stars: CheckResult,
    pub contributors: CheckResult,
    pub age: CheckResult,
    /// Domain registration is evidence for review, not proof of service age.
    pub domain: Option<String>,
    /// Explains checks that require manual review.
    pub note: Option<String>,
}

impl ToolReport {
    pub const fn has_nonpassing_checks(&self) -> bool {
        !self.stars.is_pass() || !self.contributors.is_pass() || !self.age.is_pass()
    }

    pub const fn should_close(&self) -> bool {
        let repository_age_failed = self.age.is_fail() && self.domain.is_none();
        self.stars.is_fail() || self.contributors.is_fail() || repository_age_failed
    }

    pub const fn status(&self) -> &'static str {
        if self.should_close() {
            "FAIL"
        } else if self.has_nonpassing_checks() {
            "REVIEW"
        } else {
            "PASS"
        }
    }
}

/// A Markdown comment; Askama derives its `Display` implementation.
#[derive(Template)]
#[template(path = "comment.md")]
pub struct Comment<'a> {
    marker: &'a str,
    reports: &'a [ToolReport],
    any_failures: bool,
    should_close: bool,
}

impl<'a> From<&'a [ToolReport]> for Comment<'a> {
    fn from(reports: &'a [ToolReport]) -> Self {
        Self {
            marker: COMMENT_MARKER,
            reports,
            any_failures: reports.iter().any(ToolReport::has_nonpassing_checks),
            should_close: reports.iter().any(ToolReport::should_close),
        }
    }
}

pub fn report_exit_code(reports: &[ToolReport]) -> u8 {
    if reports.iter().any(ToolReport::should_close) {
        2
    } else {
        u8::from(reports.iter().any(ToolReport::has_nonpassing_checks))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

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
        let comment = Comment::from(reports.as_slice()).render()?;
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
            let comment = Comment::from(reports.as_slice()).render()?;
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
            let comment = Comment::from(reports.as_slice()).render()?;
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
            let comment = Comment::from(reports.as_slice()).render()?;
            assert!(comment.contains("Homepage domain age"));
            assert!(comment.contains("https://rdap.org/domain/battletest.dev"));
            assert!(!comment.contains("closing this pull request"));
            assert!(comment.contains("needs manual review"));
        }
        Ok(())
    }
    #[test]
    fn render_comment_no_files() -> Result<()> {
        let template = Comment::from([].as_slice());
        let comment = template.render()?;
        assert_eq!(template.to_string(), comment);
        assert!(comment.contains("No new tool files detected"));
        Ok(())
    }

    #[test]
    fn render_comment_contains_marker() -> Result<()> {
        let comment = Comment::from([].as_slice()).render()?;
        assert!(comment.contains(COMMENT_MARKER));
        Ok(())
    }

    #[test]
    fn all_check_combinations_preserve_status_comment_and_exit_code() -> Result<()> {
        fn check(state: u8) -> CheckResult {
            match state {
                0 => CheckResult::Pass("verified".into()),
                1 => CheckResult::Fail("below minimum".into()),
                _ => CheckResult::Skip("unavailable".into()),
            }
        }
        for domain in [None, Some("example.com")] {
            for stars in 0..3 {
                for contributors in 0..3 {
                    for age in 0..3 {
                        let report = ToolReport {
                            stars: check(stars),
                            contributors: check(contributors),
                            age: check(age),
                            domain: domain.map(str::to_owned),
                            ..passing_report()
                        };
                        let close =
                            stars == 1 || contributors == 1 || (age == 1 && domain.is_none());
                        let review = stars != 0 || contributors != 0 || age != 0;
                        let (status, exit) = if close {
                            ("FAIL", 2)
                        } else if review {
                            ("REVIEW", 1)
                        } else {
                            ("PASS", 0)
                        };
                        assert_eq!(report.status(), status);
                        let reports = [passing_report(), report];
                        assert_eq!(report_exit_code(&reports), exit);
                        let template = Comment::from(reports.as_slice());
                        let comment = template.render()?;
                        assert_eq!(template.to_string(), comment);
                        assert!(comment.starts_with(COMMENT_MARKER));
                        assert_eq!(comment.contains("closing this pull request"), close);
                        assert_eq!(comment.contains("needs manual review"), review && !close);
                        assert_eq!(
                            comment.contains("All tool eligibility criteria passed"),
                            !review
                        );
                    }
                }
            }
        }
        Ok(())
    }
}
