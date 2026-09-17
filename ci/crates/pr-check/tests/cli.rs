use std::process::Command;

#[test]
fn unrelated_and_traversing_paths_are_not_loaded() -> std::io::Result<()> {
    let output = Command::new(env!("CARGO_BIN_EXE_pr-check"))
        .args(["README.md", "data/tools/../outside.yml"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("GITHUB_TOKEN", "unused-test-token")
        .env_remove("COMMENT_OUTPUT_FILE")
        .output()?;
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("Nothing to check"));
    Ok(())
}

#[test]
fn missing_catalog_file_reports_its_path_without_closing() -> std::io::Result<()> {
    let output = Command::new(env!("CARGO_BIN_EXE_pr-check"))
        .arg("data/tools/missing.yml")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("GITHUB_TOKEN", "unused-test-token")
        .env_remove("COMMENT_OUTPUT_FILE")
        .output()?;
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Cannot open data/tools/missing.yml"));
    Ok(())
}

#[test]
fn help_and_version_do_not_require_credentials() -> std::io::Result<()> {
    for (flag, expected) in [("--help", "Usage:"), ("--version", "pr-check")] {
        let output = Command::new(env!("CARGO_BIN_EXE_pr-check"))
            .arg(flag)
            .env_remove("GITHUB_TOKEN")
            .output()?;
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains(expected));
    }
    Ok(())
}

#[test]
fn invalid_arguments_never_use_the_pr_closure_exit_code() -> std::io::Result<()> {
    let output = Command::new(env!("CARGO_BIN_EXE_pr-check"))
        .arg("--unknown")
        .env_remove("GITHUB_TOKEN")
        .output()?;
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("--unknown"));
    Ok(())
}
