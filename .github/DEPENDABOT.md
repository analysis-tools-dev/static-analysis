# Dependency updates

Dependabot checks for routine Cargo and GitHub Actions updates every Monday at
11:00 UTC. Minor and patch updates are grouped by ecosystem. Security updates
have separate groups and aren't held for the weekly version-update schedule.
Major updates stay separate and require a manual merge.

The `dependabot-merge` job in `ci.yml` runs only after both CI jobs succeed:
`readme-check` and `render` (formatting, Clippy, tests, and rendering). It verifies
Dependabot's metadata, including the highest update type in a group, then
squash-merges minor and patch updates. It checks that the PR head hasn't changed
since that CI run. The merge job never checks out or executes PR code.

This uses the existing `AUTOMERGE_TOKEN` Dependabot secret. The token needs
permission to merge PRs, including PRs that update workflow files. If it expires,
replace it under **Settings → Secrets and variables → Dependabot**. Don't replace
it with `GITHUB_TOKEN`: merges made with that token won't trigger the subsequent
push workflow.

After a merge, the existing `Render` GitHub Actions workflow regenerates the
list, commits generated changes, and sends a `rebuild` event to the separate
`analysis-tools-dev/website-next` repository. Website hosting is unchanged.

No GitHub App, branch ruleset, or native **Allow auto-merge** setting is needed.
CI gates these automated dependency merges; it doesn't enforce checks on human
merges. If branch protection is added later, account for the list-generation
workflow's direct pushes to `master` before enabling it.
