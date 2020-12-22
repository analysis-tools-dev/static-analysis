use anyhow::{Context, Result};
use askama::Template;
use pico_args::Arguments;
use render::types::{Entry, ParsedEntry, Tags};
use render::{check_deprecated, group};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::PathBuf;

struct Args {
    tags: PathBuf,
    tools: PathBuf,
    out: PathBuf,
    skip_deprecated: bool,
}

fn parse_path(s: &OsStr) -> Result<PathBuf> {
    Ok(s.into())
}

fn read_tags(path: PathBuf) -> Result<Tags> {
    let f = std::fs::File::open(path)?;
    Ok(serde_yaml::from_reader(f)?)
}

fn read_tools(path: PathBuf) -> Result<Vec<ParsedEntry>> {
    let dir: std::fs::ReadDir = std::fs::read_dir(path)?;

    let files = dir
        .map(|res| res.map(|e| e.path()))
        .filter(|x| match x {
            Ok(pb) => pb.extension().and_then(OsStr::to_str) == Some("yml"),
            Err(_) => false,
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    files
        .iter()
        .inspect(|p| println!("Checking {}", p.display()))
        .map(|p| {
            let file = std::fs::File::open(p)?;
            let entry: ParsedEntry = serde_yaml::from_reader(file)?;
            Ok(entry)
        })
        .collect::<Result<Vec<ParsedEntry>, _>>()
}

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    let args = Args {
        tags: args.value_from_os_str("--tags", parse_path)?,
        tools: args.value_from_os_str("--tools", parse_path)?,
        out: args.value_from_os_str("--out", parse_path)?,
        skip_deprecated: args.contains("--skip-deprecated"),
    };

    let tags = read_tags(args.tags)?;

    let parsed_tools = read_tools(args.tools)?;
    let tools: Result<Vec<Entry>> = parsed_tools
        .into_iter()
        .map(|t| Entry::from_parsed(t, &tags))
        .collect();
    let mut tools = tools?;
    tools.sort();

    if !args.skip_deprecated {
        if let Ok(token) = env::var("GITHUB_TOKEN") {
            check_deprecated(token, &mut tools)?;
        }
    }

    let catalog = group(&tags, &tools)?;
    fs::write(&args.out, catalog.render()?).context("Cannot write")?;
    Ok(())
}
