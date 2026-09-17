//! Report status, Markdown rendering, and workflow exit-code contract.

use askama::Template;
use std::process::ExitCode;

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

/// The ordered outcomes of all tool checks.
#[derive(Debug, Default)]
pub struct Reports(Vec<ToolReport>);

impl Reports {
    pub fn exit_code(&self) -> ExitCode {
        if self.0.iter().any(ToolReport::should_close) {
            // The workflow reserves code 2 for verified failures that close the PR.
            ExitCode::from(2)
        } else if self.0.iter().any(ToolReport::has_nonpassing_checks) {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    }
}

impl Extend<ToolReport> for Reports {
    fn extend<T: IntoIterator<Item = ToolReport>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl FromIterator<ToolReport> for Reports {
    fn from_iter<T: IntoIterator<Item = ToolReport>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
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

impl<'a> From<&'a Reports> for Comment<'a> {
    fn from(reports: &'a Reports) -> Self {
        Self {
            marker: COMMENT_MARKER,
            reports: &reports.0,
            any_failures: reports.0.iter().any(ToolReport::has_nonpassing_checks),
            should_close: reports.0.iter().any(ToolReport::should_close),
        }
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
    fn collect_and_extend_preserve_input_order() {
        let report = |name: &str| ToolReport {
            name: name.into(),
            ..passing_report()
        };
        let mut reports: Reports = [report("First"), report("Second")].into_iter().collect();
        reports.extend([report("Third"), report("Fourth")]);
        reports.extend([]);
        let comment = Comment::from(&reports);
        let names: Vec<_> = comment
            .reports
            .iter()
            .map(|report| report.name.as_str())
            .collect();
        assert_eq!(names, ["First", "Second", "Third", "Fourth"]);
        assert_eq!(reports.exit_code(), ExitCode::SUCCESS);
    }

    #[test]
    fn empty_reports_have_no_failures() -> Result<()> {
        for mut reports in [Reports::default(), std::iter::empty().collect()] {
            reports.extend([]);
            assert_eq!(reports.exit_code(), ExitCode::SUCCESS);
            let comment = Comment::from(&reports);
            assert!(comment.reports.is_empty());
            assert!(!comment.any_failures);
            assert!(!comment.should_close);
            assert!(comment.render()?.contains("No new tool files detected"));
        }
        Ok(())
    }

    #[test]
    fn passing_tools_do_not_close_pr() -> Result<()> {
        let reports: Reports = std::iter::once(passing_report()).collect();
        assert_eq!(reports.exit_code(), ExitCode::SUCCESS);
        let comment = Comment::from(&reports).render()?;
        assert!(comment.contains("All tool eligibility criteria passed"));
        assert!(!comment.contains("closing this pull request"));
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
            let reports: Reports = [passing_report(), report].into_iter().collect();
            assert_eq!(reports.exit_code(), ExitCode::from(2));
            let comment = Comment::from(&reports).render()?;
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
            let reports = Reports::from_iter([report]);
            assert_eq!(reports.exit_code(), ExitCode::FAILURE);
            let comment = Comment::from(&reports).render()?;
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
        assert_eq!(Reports::from_iter([report]).exit_code(), ExitCode::from(2));
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
            let reports = Reports::from_iter([report]);
            assert_eq!(reports.exit_code(), ExitCode::FAILURE);
            let comment = Comment::from(&reports).render()?;
            assert!(comment.contains("Homepage domain age"));
            assert!(comment.contains("https://rdap.org/domain/battletest.dev"));
            assert!(!comment.contains("closing this pull request"));
            assert!(comment.contains("needs manual review"));
        }
        Ok(())
    }
    #[test]
    fn render_comment_no_files() -> Result<()> {
        let reports = Reports::from_iter([]);
        let template = Comment::from(&reports);
        let comment = template.render()?;
        assert_eq!(template.to_string(), comment);
        assert!(comment.contains("No new tool files detected"));
        Ok(())
    }

    #[test]
    fn render_comment_contains_marker() -> Result<()> {
        let comment = Comment::from(&Reports::from_iter([])).render()?;
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
                        let reports: Reports = [passing_report(), report].into_iter().collect();
                        assert_eq!(reports.exit_code(), ExitCode::from(exit));
                        let template = Comment::from(&reports);
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
