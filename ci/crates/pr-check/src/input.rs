//! Catalog input paths and tool loading.

use anyhow::{Context, Result, ensure};
use github_repo::ToolSource;
use serde::Deserialize;
use std::fmt;
use std::path::{Component, PathBuf};

/// A relative YAML path under `data/tools`, with no parent-directory traversal.
/// This classifies a catalog path; it does not resolve symlinks or guarantee existence.
#[derive(Debug)]
pub struct ToolPath(PathBuf);

impl TryFrom<PathBuf> for ToolPath {
    type Error = anyhow::Error;

    fn try_from(path: PathBuf) -> Result<Self> {
        ensure!(
            path.starts_with("data/tools")
                && !path
                    .components()
                    .any(|component| component == Component::ParentDir)
                && matches!(
                    path.extension().and_then(|ext| ext.to_str()),
                    Some("yml" | "yaml")
                ),
            "Expected a relative YAML path under data/tools: {}",
            path.display()
        );
        Ok(Self(path))
    }
}

impl fmt::Display for ToolPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.display().fmt(f)
    }
}

/// The tool fields required by the contribution checks.
#[derive(Debug, Deserialize)]
pub struct ToolEntry {
    pub name: String,
    pub source: Option<ToolSource>,
    pub homepage: Option<String>,
}

impl ToolEntry {
    /// Reads a catalog entry relative to the current working directory.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or parsed.
    pub fn read(path: &ToolPath) -> Result<Self> {
        let file = std::fs::File::open(&path.0).with_context(|| format!("Cannot open {path}"))?;
        serde_saphyr::from_reader(file).with_context(|| format!("Cannot parse {path}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parses_only_catalog_yaml_paths() -> Result<()> {
        for path in [
            "data/tools/tool.yml",
            "data/tools/tool.yaml",
            "data/tools/nested/tool.yml",
        ] {
            let parsed = ToolPath::try_from(PathBuf::from(path))?;
            assert_eq!(parsed.to_string(), path);
        }
        for path in [
            "README.md",
            "data/tags.yml",
            "data/tools-extra/tool.yml",
            "data/tools/tool.YML",
            "data/tools/tool.json",
            "/data/tools/tool.yml",
            "data/tools/../outside.yml",
            "data/tools/nested/../../outside.yml",
        ] {
            assert!(ToolPath::try_from(PathBuf::from(path)).is_err(), "{path}");
        }
        Ok(())
    }

    #[test]
    fn parses_catalog() -> Result<()> {
        let tools = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data/tools");
        let mut count = 0;
        for entry in std::fs::read_dir(tools)? {
            let entry = entry?;
            let relative = Path::new("data/tools").join(entry.file_name());
            if ToolPath::try_from(relative).is_ok() {
                let tool: ToolEntry =
                    serde_saphyr::from_reader(std::fs::File::open(entry.path())?)?;
                assert!(!tool.name.is_empty(), "{}", entry.path().display());
                count += 1;
            }
        }
        assert!(count > 0);
        Ok(())
    }

    #[test]
    fn tool_sources_are_classified_during_deserialization() -> Result<()> {
        let tool: ToolEntry =
            serde_saphyr::from_str("name: Example\nsource: http://github.com/Owner/Repo///\n")?;
        let Some(ToolSource::Github(repo)) = tool.source else {
            anyhow::bail!("Expected a GitHub repository");
        };
        assert_eq!(repo.to_string(), "Owner/Repo");
        assert_eq!(repo.as_str(), "http://github.com/Owner/Repo///");
        for source in [
            "https://gitlab.com/owner/repo",
            "https://github.com/owner/repo/tree/main",
            "",
        ] {
            let yaml = format!("name: Example\nsource: '{source}'\n");
            let tool: ToolEntry = serde_saphyr::from_str(&yaml)?;
            assert!(matches!(tool.source, Some(ToolSource::Other(ref value)) if value == source));
        }
        for yaml in ["name: Example\n", "name: Example\nsource: null\n"] {
            assert!(serde_saphyr::from_str::<ToolEntry>(yaml)?.source.is_none());
        }
        Ok(())
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
