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

mod checks;
mod criteria;
mod network;
mod report;

use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use criteria::ToolEntry;
use network::{GithubClient, check_tool};
use report::{render_comment, report_exit_code};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Changed files to check; only YAML files under data/tools are inspected.
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,
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

fn is_tool_path(path: &Path) -> bool {
    path.starts_with("data/tools")
        && matches!(
            path.extension().and_then(|extension| extension.to_str()),
            Some("yml" | "yaml")
        )
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<ExitCode> {
    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(error) => {
            // Exit code 2 tells the workflow to close the PR, so CLI errors must use 1.
            let code = u8::from(error.use_stderr());
            error.print()?;
            return Ok(ExitCode::from(code));
        }
    };
    let token = env::var("GITHUB_TOKEN").context("GITHUB_TOKEN not set")?;

    let client = GithubClient::new(token)?;

    let mut reports = Vec::new();
    for path in args.files.iter().filter(|path| is_tool_path(path)) {
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
    }

    Ok(ExitCode::from(exit_code))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_files_and_provides_help() -> Result<()> {
        let args = Args::try_parse_from(["pr-check", "data/tools/example.yml", "README.md"])?;
        assert_eq!(
            args.files,
            [
                PathBuf::from("data/tools/example.yml"),
                PathBuf::from("README.md")
            ]
        );
        assert!(Args::try_parse_from(["pr-check"])?.files.is_empty());
        for (flag, kind) in [
            ("--help", clap::error::ErrorKind::DisplayHelp),
            ("--version", clap::error::ErrorKind::DisplayVersion),
            ("--unknown", clap::error::ErrorKind::UnknownArgument),
        ] {
            let error = Args::try_parse_from(["pr-check", flag])
                .err()
                .context("Expected help or error")?;
            assert_eq!(error.kind(), kind);
        }
        Ok(())
    }

    #[test]
    fn parses_catalog() -> Result<()> {
        let tools = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/tools");
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
    fn tool_path_filter_accepts_only_catalog_yaml_arguments() {
        for path in [
            "data/tools/tool.yml",
            "data/tools/tool.yaml",
            "data/tools/nested/tool.yml",
        ] {
            assert!(is_tool_path(Path::new(path)), "{path}");
        }
        for path in [
            "README.md",
            "data/tags.yml",
            "data/tools-extra/tool.yml",
            "data/tools/tool.YML",
            "data/tools/tool.json",
            "/data/tools/tool.yml",
        ] {
            assert!(!is_tool_path(Path::new(path)), "{path}");
        }
    }

    #[test]
    fn tool_yaml_keeps_optional_fields_and_ignores_unrelated_metadata() -> Result<()> {
        let tool: ToolEntry =
            serde_saphyr::from_str("name: Example\nlicense: MIT\ntags: [rust]\n")?;
        assert_eq!(tool.name, "Example");
        assert!(tool.source.is_none());
        assert!(tool.homepage.is_none());
        assert!(serde_saphyr::from_str::<ToolEntry>("source: https://example.com").is_err());
        Ok(())
    }
}
