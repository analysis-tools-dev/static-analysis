use anyhow::{Context, Result, ensure};
use askama::Template;
use pico_args::Arguments;
use render::types::{Collection, Entry, ParsedEntry, Tag, Tags, Type};
use render::{check_deprecated, create_api, create_catalog};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use slug::slugify;
use std::collections::BTreeMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Args {
    tags: PathBuf,
    tools: PathBuf,
    collections: PathBuf,
    md_out: PathBuf,
    json_out: PathBuf,
    skip_deprecated: bool,
}

impl Args {
    fn parse(mut args: Arguments) -> Result<Self> {
        let parsed = Self {
            tags: args.value_from_os_str("--tags", parse_path)?,
            tools: args.value_from_os_str("--tools", parse_path)?,
            collections: args.value_from_os_str("--collections", parse_path)?,
            md_out: args.value_from_os_str("--md-out", parse_path)?,
            json_out: args.value_from_os_str("--json-out", parse_path)?,
            skip_deprecated: args.contains("--skip-deprecated"),
        };
        let remaining = args.finish();
        ensure!(remaining.is_empty(), "Unexpected arguments: {remaining:?}");
        Ok(parsed)
    }
}

// `pico_args::value_from_os_str` requires a fallible parser callback.
#[allow(clippy::unnecessary_wraps)]
fn parse_path(s: &OsStr) -> Result<PathBuf> {
    Ok(s.into())
}

fn read_yaml<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let file = fs::File::open(path).with_context(|| format!("Cannot open {}", path.display()))?;
    serde_saphyr::from_reader(file).with_context(|| format!("Cannot parse {}", path.display()))
}

fn read_entries<T: DeserializeOwned>(path: &Path) -> Result<Vec<T>> {
    let dir = fs::read_dir(path).with_context(|| format!("Cannot read {}", path.display()))?;
    let mut files = dir
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<io::Result<Vec<_>>>()
        .with_context(|| format!("Cannot list {}", path.display()))?;
    files.retain(|path| path.extension().is_some_and(|extension| extension == "yml"));

    files
        .iter()
        .map(|path| {
            println!("Checking {}", path.display());
            read_yaml(path)
        })
        .collect()
}

#[derive(Deserialize)]
struct CachedTool {
    deprecated: Option<bool>,
}

/// Reuses cached markers without overriding explicit YAML decisions.
fn backfill_deprecated(tools: &mut [Entry], path: &Path) -> Result<()> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => return Err(error).with_context(|| format!("Cannot open {}", path.display())),
    };
    let cached: BTreeMap<String, CachedTool> = serde_json::from_reader(io::BufReader::new(file))
        .with_context(|| format!("Cannot parse {}", path.display()))?;
    apply_deprecation_cache(tools, &cached);
    Ok(())
}

fn apply_deprecation_cache(tools: &mut [Entry], cached: &BTreeMap<String, CachedTool>) {
    for tool in tools.iter_mut().filter(|tool| tool.deprecated.is_none()) {
        tool.deprecated = cached
            .get(&slugify(&tool.name))
            .and_then(|old| old.deprecated);
    }
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let json = serde_json::to_vec_pretty(value)?;
    fs::write(path, json).with_context(|| format!("Cannot write {}", path.display()))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse(Arguments::from_env())?;
    let tags: Tags = read_yaml(&args.tags)?;

    let mut collections: Vec<Collection> = read_entries(&args.collections)?;
    collections.sort_by_cached_key(|collection| collection.name.to_lowercase());

    let parsed_tools: Vec<ParsedEntry> = read_entries(&args.tools)?;
    let mut tools = parsed_tools
        .into_iter()
        .map(|tool| Entry::from_parsed(tool, &tags))
        .collect::<Result<Vec<_>>>()?;
    tools.sort();

    let should_check_deprecation = !args.skip_deprecated;
    let github_token = env::var("GITHUB_TOKEN");

    let cache_path = Path::new("data/api/tools.json");
    match (should_check_deprecation, github_token) {
        (true, Ok(token)) => {
            println!("Checking for deprecated entries on GitHub. This might take a while...");
            check_deprecated(&token, &mut tools).await?;
        }
        (true, Err(_)) => {
            eprintln!("No GITHUB_TOKEN environment variable found. Reusing old deprecation data.");
            backfill_deprecated(&mut tools, cache_path)?;
        }
        (false, _) => backfill_deprecated(&mut tools, cache_path)?,
    }

    let (languages, other_tags): (Vec<Tag>, Vec<Tag>) =
        tags.into_iter().partition(|tag| tag.kind == Type::Language);

    let catalog = create_catalog(&tools, &languages, &other_tags, collections);
    fs::write(&args.md_out, catalog.render()?)
        .with_context(|| format!("Cannot write Markdown output to {}", args.md_out.display()))?;

    let api = create_api(tools, &languages, &other_tags);

    write_json(&args.json_out.join("tools.json"), &api)?;
    let tags_json = BTreeMap::from([("languages", languages), ("other", other_tags)]);
    write_json(&args.json_out.join("tags.json"), &tags_json)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cli_args() -> Vec<std::ffi::OsString> {
        [
            "--tags",
            "tags.yml",
            "--tools",
            "tools",
            "--collections",
            "collections",
            "--md-out",
            "README.md",
            "--json-out",
            "api",
        ]
        .into_iter()
        .map(Into::into)
        .collect()
    }

    #[test]
    fn parses_cli_and_rejects_unknown_or_missing_arguments() -> Result<()> {
        let mut args = cli_args();
        args.push("--skip-deprecated".into());
        let parsed = Args::parse(Arguments::from_vec(args))?;
        assert!(parsed.skip_deprecated);
        assert_eq!(parsed.collections, Path::new("collections"));

        let mut args = cli_args();
        args.push("--skip-deprected".into());
        let error = Args::parse(Arguments::from_vec(args))
            .err()
            .context("Expected invalid argument")?;
        assert!(error.to_string().contains("--skip-deprected"));
        assert!(Args::parse(Arguments::from_vec(vec![])).is_err());
        Ok(())
    }

    #[test]
    fn cached_deprecation_never_overrides_explicit_markers() -> Result<()> {
        let fixture = r#"{"name":"Example Tool","categories":[],"tags":[],"license":"MIT","types":[],"homepage":"https://example.com","description":"Example"}"#;
        for cached_marker in [Some(true), Some(false), None] {
            let cached = BTreeMap::from([(
                "example-tool".into(),
                CachedTool {
                    deprecated: cached_marker,
                },
            )]);
            for explicit in [Some(true), Some(false), None] {
                let mut tool: Entry = serde_json::from_str(fixture)?;
                tool.deprecated = explicit;
                apply_deprecation_cache(std::slice::from_mut(&mut tool), &cached);
                assert_eq!(tool.deprecated, explicit.or(cached_marker));
            }
        }
        let mut tool: Entry = serde_json::from_str(fixture)?;
        apply_deprecation_cache(std::slice::from_mut(&mut tool), &BTreeMap::new());
        assert_eq!(tool.deprecated, None);
        Ok(())
    }

    #[test]
    fn file_errors_include_the_source_path() -> Result<()> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
        let error = read_entries::<Collection>(&path)
            .err()
            .context("Expected directory error")?;
        assert!(error.to_string().contains(&path.display().to_string()));
        let error = backfill_deprecated(&mut [], &path)
            .err()
            .context("Expected JSON error")?;
        assert!(error.to_string().contains(&path.display().to_string()));
        Ok(())
    }

    #[test]
    fn parses_catalog() -> Result<()> {
        let data = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../data");
        let tags: Tags = read_yaml(&data.join("tags.yml"))?;
        let tools: Vec<ParsedEntry> = read_entries(&data.join("tools"))?;
        let collections: Vec<Collection> = read_entries(&data.join("collections"))?;
        assert!(!collections.is_empty());
        let catalog = create_catalog(&[], &[], &[], collections);
        let markdown = catalog.render()?;
        for collection in &catalog.collections {
            assert!(!collection.name.is_empty());
            reqwest::Url::parse(&collection.homepage)?;
            assert!(!collection.description.is_empty());
            assert!(markdown.contains(&format!(
                "- [{}]({}) — {}",
                collection.name, collection.homepage, collection.description
            )));
        }
        assert!(!tags.is_empty());
        assert!(!tools.is_empty());
        for tool in tools {
            Entry::from_parsed(tool, &tags)?;
        }
        Ok(())
    }
}
