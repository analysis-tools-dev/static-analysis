//! Individual contribution checks over already-fetched metadata.

use anyhow::{Context, Result};
use chrono::{DateTime, Months, Utc};

use crate::criteria::RepoInfo;
use crate::report::CheckResult;

const MIN_STARS: u64 = 20;
const MIN_CONTRIBUTORS: usize = 2;
const MIN_AGE_MONTHS: u32 = 6;

/// Evaluates a criterion without I/O. Unavailable evidence produces a skipped check.
pub trait Check {
    /// Returns an error if evaluation cannot complete, such as invalid date arithmetic.
    fn check(&self) -> Result<CheckResult>;
}

pub struct Stars<'a> {
    pub repository: &'a Result<Option<RepoInfo>>,
}

impl Check for Stars<'_> {
    fn check(&self) -> Result<CheckResult> {
        Ok(match self.repository {
            Ok(Some(info)) => {
                let stars = info.stargazers_count;
                if stars >= MIN_STARS {
                    CheckResult::Pass(format!("{stars} stars"))
                } else {
                    CheckResult::Fail(format!("{stars} stars (minimum is {MIN_STARS})"))
                }
            }
            Ok(None) => CheckResult::Skip("repository not found".into()),
            Err(error) => CheckResult::Skip(format!("Could not fetch repo info: {error}")),
        })
    }
}

pub struct Contributors<'a> {
    pub count: &'a Result<Option<usize>>,
}

impl Check for Contributors<'_> {
    fn check(&self) -> Result<CheckResult> {
        Ok(match self.count {
            Ok(Some(count)) => {
                if *count >= MIN_CONTRIBUTORS {
                    CheckResult::Pass(format!("{count} human contributors"))
                } else {
                    CheckResult::Fail(format!(
                        "{count} human contributor(s) (minimum is {MIN_CONTRIBUTORS})"
                    ))
                }
            }
            Ok(None) => CheckResult::Skip("repository not found".into()),
            Err(error) => CheckResult::Skip(format!("Could not fetch contributors: {error}")),
        })
    }
}

pub struct RepositoryAge<'a> {
    pub repository: &'a Result<Option<RepoInfo>>,
    pub now: DateTime<Utc>,
}

impl Check for RepositoryAge<'_> {
    fn check(&self) -> Result<CheckResult> {
        Ok(match self.repository {
            Ok(Some(info)) => {
                let minimum_created_at =
                    self.now
                        .checked_sub_months(Months::new(MIN_AGE_MONTHS))
                        .context("Current date cannot be shifted back by six months")?;
                let days = self.now.signed_duration_since(info.created_at).num_days();
                if info.created_at <= minimum_created_at {
                    CheckResult::Pass(format!("created {days} days ago (at least 6 months)"))
                } else {
                    let eligible_at = info.created_at
                        .checked_add_months(Months::new(MIN_AGE_MONTHS))
                        .with_context(|| format!(
                            "Repository creation date {} cannot be shifted forward by {MIN_AGE_MONTHS} months",
                            info.created_at
                        ))?;
                    let remaining = eligible_at
                        .signed_duration_since(self.now)
                        .num_days()
                        .max(1);
                    CheckResult::Fail(format!(
                        "created {days} days ago, needs {remaining} more days to meet the 6-month minimum"
                    ))
                }
            }
            Ok(None) => CheckResult::Skip("repository not found".into()),
            Err(_) => CheckResult::Skip("Could not determine age (repo info unavailable)".into()),
        })
    }
}

pub struct DomainAge<'a> {
    pub domain: &'a str,
    pub registration: &'a Result<Option<DateTime<Utc>>>,
    pub now: DateTime<Utc>,
}

impl Check for DomainAge<'_> {
    fn check(&self) -> Result<CheckResult> {
        let registered = match self.registration {
            Ok(Some(registered)) => *registered,
            Ok(None) => {
                return Ok(CheckResult::Skip(
                    "Domain registration date unavailable; manual review required".into(),
                ));
            }
            Err(error) => {
                return Ok(CheckResult::Skip(format!(
                    "Could not check domain registration: {error}"
                )));
            }
        };
        let Some(eligible) = registered.checked_add_months(Months::new(MIN_AGE_MONTHS)) else {
            return Ok(CheckResult::Skip("Invalid domain registration date".into()));
        };
        if registered > self.now {
            return Ok(CheckResult::Skip(
                "Domain registration date is in the future; manual review required".into(),
            ));
        }
        if self.now < eligible {
            Ok(CheckResult::Fail(format!(
                "The homepage domain `{}` was registered on {} and reaches the six-month minimum on {}.",
                self.domain,
                registered.format("%B %-d, %Y"),
                eligible.format("%B %-d, %Y")
            )))
        } else {
            Ok(CheckResult::Pass(format!(
                "Domain registered on {} (at least six months ago). Service age still requires manual review.",
                registered.format("%B %-d, %Y")
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_age_uses_six_calendar_months() -> Result<()> {
        let registered = "2026-05-01T20:44:07Z".parse::<DateTime<Utc>>()?;
        let before = "2026-11-01T20:44:06Z".parse::<DateTime<Utc>>()?;
        let boundary = "2026-11-01T20:44:07Z".parse::<DateTime<Utc>>()?;
        let check = |registered, now| {
            DomainAge {
                domain: "battletest.dev",
                registration: &Ok(Some(registered)),
                now,
            }
            .check()
        };
        let result = check(registered, before)?;
        assert!(result.is_fail());
        assert!(result.message().contains("registered on May 1, 2026"));
        assert!(result.message().contains("minimum on November 1, 2026"));
        assert!(check(registered, boundary)?.is_pass());
        assert!(matches!(check(boundary, registered)?, CheckResult::Skip(_)));
        Ok(())
    }

    #[test]
    fn all_checks_treat_unavailable_evidence_as_unverified() -> Result<()> {
        let now = "2026-09-17T00:00:00Z".parse()?;
        for unavailable in [false, true] {
            let repository = if unavailable {
                Err(anyhow::anyhow!("API unavailable"))
            } else {
                Ok(None)
            };
            let count = if unavailable {
                Err(anyhow::anyhow!("API unavailable"))
            } else {
                Ok(None)
            };
            let registration = if unavailable {
                Err(anyhow::anyhow!("RDAP unavailable"))
            } else {
                Ok(None)
            };
            let checks: [&dyn Check; 4] = [
                &Stars {
                    repository: &repository,
                },
                &Contributors { count: &count },
                &RepositoryAge {
                    repository: &repository,
                    now,
                },
                &DomainAge {
                    domain: "example.com",
                    registration: &registration,
                    now,
                },
            ];
            for check in checks {
                assert!(matches!(check.check()?, CheckResult::Skip(_)));
            }
        }
        Ok(())
    }
}
