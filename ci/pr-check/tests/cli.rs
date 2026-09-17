use std::process::Command;

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
