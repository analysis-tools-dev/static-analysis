//! Catalog input paths and tool loading.

use anyhow::{Context, Result, ensure};
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
    pub source: Option<String>,
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
