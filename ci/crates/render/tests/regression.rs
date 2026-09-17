use anyhow::{Context, Result};
use github_repo::{GithubRepo, ToolSource};
use serde_json::json;
use std::collections::{BTreeMap, BTreeSet};

use render::types::{ApiEntry, Catalog, Entry, ParsedEntry, Tag, ToolType, Type};
use render::{create_api, create_catalog, format_stats, stats};

fn tag(value: &str, kind: Type) -> Tag {
    Tag {
        name: value.into(),
        value: value.into(),
        kind,
        include_multi: false,
    }
}

fn parsed() -> Result<ParsedEntry> {
    Ok(serde_json::from_value(json!({
        "name": "Example Tool",
        "categories": ["linter"],
        "tags": ["rust"],
        "license": "MIT / Apache License",
        "types": ["cli"],
        "homepage": "https://example.com",
        "description": "Example description"
    }))?)
}

fn assert_source_preserved(tool: ParsedEntry, tags: &[Tag], raw: Option<&str>) -> Result<()> {
    let expected = raw.map(ToolSource::from);
    assert_eq!(tool.source, expected);
    assert_eq!(serde_json::to_value(&tool)?["source"], json!(raw));
    let entry = Entry::from_parsed(tool, tags)?;
    assert_eq!(entry.source, expected);
    let encoded = serde_json::to_string(&entry)?;
    assert_eq!(serde_json::from_str::<Entry>(&encoded)?.source, expected);
    assert_eq!(serde_saphyr::from_str::<Entry>(&encoded)?.source, expected);
    let api = create_api(vec![entry], tags, &[]);
    let api_entry = api.values().next().context("Expected one API entry")?;
    assert_eq!(api_entry.source, expected);
    assert_eq!(serde_json::to_value(api_entry)?["source"], json!(raw));
    let encoded = serde_json::to_string(api_entry)?;
    assert_eq!(serde_json::from_str::<ApiEntry>(&encoded)?.source, expected);
    assert_eq!(
        serde_saphyr::from_str::<ApiEntry>(&encoded)?.source,
        expected
    );
    Ok(())
}

#[test]
fn source_variants_and_absence_survive_normalization_and_api_output() -> Result<()> {
    for (raw, github) in [
        (Some("http://github.com/Owner/Repo///"), true),
        (Some("https://github.com/owner/repo?tab=readme"), true),
        (Some("https://gitlab.com/Owner/Repo/"), false),
        (Some("https://github.com/owner/repo/tree/main"), false),
        (Some("https://github.com/owner/"), false),
        (Some("not a URL"), false),
        (Some(""), false),
        (None, false),
    ] {
        let mut value = serde_json::to_value(parsed()?)?;
        value["source"] = json!(raw);
        let encoded = serde_json::to_string(&value)?;
        for tool in [
            serde_json::from_str::<ParsedEntry>(&encoded)?,
            serde_saphyr::from_str::<ParsedEntry>(&encoded)?,
        ] {
            assert_eq!(matches!(tool.source, Some(ToolSource::Github(_))), github);
            assert_source_preserved(tool, &[tag("rust", Type::Language)], raw)?;
        }
    }
    let mut value = serde_json::to_value(parsed()?)?;
    value
        .as_object_mut()
        .context("Expected object")?
        .remove("source");
    let encoded = serde_json::to_string(&value)?;
    for tool in [
        serde_json::from_str::<ParsedEntry>(&encoded)?,
        serde_saphyr::from_str::<ParsedEntry>(&encoded)?,
    ] {
        assert_source_preserved(tool, &[tag("rust", Type::Language)], None)?;
    }
    Ok(())
}

#[test]
fn full_catalog_preserves_original_sources_through_api_output() -> Result<()> {
    #[derive(serde::Deserialize)]
    struct RawSource {
        source: Option<String>,
    }

    let data = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../data");
    let tags: Vec<Tag> = serde_saphyr::from_str(&std::fs::read_to_string(data.join("tags.yml"))?)?;
    let mut counts = [0; 3];
    for file in std::fs::read_dir(data.join("tools"))? {
        let path = file?.path();
        if path.extension().is_none_or(|extension| extension != "yml") {
            continue;
        }
        let yaml = std::fs::read_to_string(&path)?;
        let raw: RawSource = serde_saphyr::from_str(&yaml)?;
        let tool: ParsedEntry = serde_saphyr::from_str(&yaml)?;
        let index = match &tool.source {
            Some(ToolSource::Github(repo)) => {
                assert_eq!(Some(repo.as_str()), raw.source.as_deref());
                0
            }
            Some(ToolSource::Other(source)) => {
                assert!(GithubRepo::try_from(source.as_str()).is_err());
                1
            }
            None => 2,
        };
        counts[index] += 1;
        assert_source_preserved(tool, &tags, raw.source.as_deref())
            .with_context(|| path.display().to_string())?;
    }
    assert!(
        counts.into_iter().all(|count| count > 0),
        "Expected GitHub, other, and absent sources"
    );
    Ok(())
}

#[test]
fn normalization_preserves_fields_and_uses_the_first_matching_tag() -> Result<()> {
    let original = parsed()?;
    let rust = tag("rust", Type::Language);
    let mut duplicate = rust.clone();
    duplicate.name = "Different metadata".into();
    let normalized = Entry::from_parsed(original.clone(), &[rust.clone(), duplicate])?;
    assert_eq!(normalized.tags.iter().collect::<Vec<_>>(), [&rust]);
    assert_eq!(normalized.types, [ToolType::Commandline].into());
    let mut expected = serde_json::to_value(original)?;
    expected["tags"] = serde_json::to_value(&normalized.tags)?;
    assert_eq!(serde_json::to_value(normalized)?, expected);
    Ok(())
}

#[test]
fn unknown_tags_are_reported_together_before_invalid_tool_types() -> Result<()> {
    let mut tool = parsed()?;
    tool.tags =
        BTreeSet::from(["z-unknown".into(), "a-unknown".into(), "rust".into()]).try_into()?;
    tool.types = ["invalid".into()].into();
    let error = Entry::from_parsed(tool, &[tag("rust", Type::Language)])
        .err()
        .context("Unknown tags should be rejected")?;
    assert_eq!(
        error.to_string(),
        "Tool 'Example Tool': Invalid tag: a-unknown\nInvalid tag: z-unknown\n  File: data/tools/example-tool.yml"
    );
    Ok(())
}

#[test]
fn tool_type_deserialization_matches_the_previous_json_conversion() -> Result<()> {
    let tags = [tag("rust", Type::Language)];
    for value in ["cli", "gui", "service", "ide-plugin", "", "CLI", "unknown"] {
        let mut tool = parsed()?;
        tool.types = [value.into()].into();
        let previous = serde_json::from_value::<ToolType>(serde_json::to_value(value)?);
        let current = Entry::from_parsed(tool, &tags).map(|entry| entry.types);
        match previous {
            Ok(kind) => assert_eq!(current?, [kind].into()),
            Err(error) => assert_eq!(
                current
                    .err()
                    .context("Invalid type should be rejected")?
                    .to_string(),
                error.to_string()
            ),
        }
    }
    Ok(())
}

#[test]
fn parsed_and_normalized_deserialization_enforce_invariants() -> Result<()> {
    let parsed = serde_json::to_value(parsed()?)?;
    let normalized = serde_json::to_value(Entry::from_parsed(
        serde_json::from_value(parsed.clone())?,
        &[tag("rust", Type::Language)],
    )?)?;
    for (original, is_normalized) in [(parsed, false), (normalized, true)] {
        let rejects = |value| {
            if is_normalized {
                serde_json::from_value::<Entry>(value).is_err()
            } else {
                serde_json::from_value::<ParsedEntry>(value).is_err()
            }
        };
        for name in [
            String::new(),
            " \t\n\u{2003}".into(),
            "a".repeat(51),
            "é".repeat(26),
        ] {
            let mut invalid = original.clone();
            invalid["name"] = json!(name);
            assert!(rejects(invalid));
        }
        for name in ["a".repeat(50), "é".repeat(25), "  Astrée  ".into()] {
            let mut valid = original.clone();
            valid["name"] = json!(name);
            let encoded = if is_normalized {
                serde_json::to_value(serde_json::from_value::<Entry>(valid.clone())?)?
            } else {
                serde_json::to_value(serde_json::from_value::<ParsedEntry>(valid.clone())?)?
            };
            assert_eq!(encoded, valid);
        }
        let mut invalid = original.clone();
        invalid["tags"] = json!([]);
        assert!(rejects(invalid));
        for field in ["name", "tags"] {
            let mut invalid = original.clone();
            invalid[field] = serde_json::Value::Null;
            assert!(rejects(invalid));
        }
    }
    Ok(())
}

#[test]
fn normalized_tags_deserialize_as_a_sorted_nonempty_set() -> Result<()> {
    let rust = tag("rust", Type::Language);
    let cpp = tag("cpp", Type::Language);
    let entry = Entry::from_parsed(parsed()?, std::slice::from_ref(&rust))?;
    let mut value = serde_json::to_value(entry)?;
    value["tags"] = json!([rust, cpp, rust]);
    let encoded = serde_json::to_string(&value)?;
    for entry in [
        serde_json::from_str::<Entry>(&encoded)?,
        serde_saphyr::from_str::<Entry>(&encoded)?,
    ] {
        assert_eq!(entry.tags.iter().collect::<Vec<_>>(), [&cpp, &rust]);
        assert_eq!(serde_json::to_value(entry.tags)?, json!([cpp, rust]));
    }
    value["tags"] = json!([]);
    assert!(serde_saphyr::from_str::<Entry>(&serde_json::to_string(&value)?).is_err());
    Ok(())
}

#[test]
fn api_preserves_configured_tag_order_duplicates_and_all_fields() -> Result<()> {
    let rust = tag("rust", Type::Language);
    let python = tag("python", Type::Language);
    let security = tag("security", Type::Other);
    let mut raw = parsed()?;
    raw.tags = BTreeSet::from(["rust".into(), "python".into(), "security".into()]).try_into()?;
    raw.source = Some("https://github.com/owner/repo".into());
    raw.pricing = Some("https://example.com/pricing".into());
    raw.plans = Some(BTreeMap::from([("free".into(), true)]));
    raw.discussion = Some("https://example.com/discussion".into());
    raw.deprecated = Some(false);
    raw.resources = Some(serde_json::from_value(
        json!([{"title": "Docs", "url": "https://example.com/docs"}]),
    )?);
    raw.reviews = Some(["https://example.com/review".into()].into());
    raw.demos = Some(["https://example.com/demo".into()].into());
    raw.wrapper = Some(true);
    let tool = Entry::from_parsed(raw, &[rust.clone(), python.clone(), security.clone()])?;
    let mut different_metadata = rust.clone();
    different_metadata.name = "Not the same tag".into();
    let api = create_api(
        vec![tool],
        &[rust.clone(), python, rust, different_metadata],
        &[security],
    );
    assert_eq!(
        serde_json::to_value(api)?,
        json!({
            "example-tool": {
                "name": "Example Tool", "categories": ["linter"],
                "languages": ["rust", "python", "rust"], "other": ["security"],
                "licenses": ["MIT / Apache License"], "types": ["cli"],
                "homepage": "https://example.com", "description": "Example description",
                "source": "https://github.com/owner/repo", "pricing": "https://example.com/pricing",
                "plans": {"free": true}, "discussion": "https://example.com/discussion",
                "deprecated": false, "resources": [{"title": "Docs", "url": "https://example.com/docs"}],
                "reviews": ["https://example.com/review"], "demos": ["https://example.com/demo"],
                "wrapper": true
            }
        })
    );
    Ok(())
}

#[test]
fn api_slug_collisions_keep_the_last_entry_and_missing_fields_stay_null() -> Result<()> {
    let tool = Entry::from_parsed(parsed()?, &[tag("rust", Type::Language)])?;
    let mut replacement = tool.clone();
    replacement.name = String::from("Example-Tool").try_into()?;
    let api = create_api(vec![tool, replacement], &[], &[]);
    assert_eq!(api.len(), 1);
    assert_eq!(api["example-tool"].name, "Example-Tool");
    let encoded = serde_json::to_value(api)?;
    for field in [
        "source",
        "pricing",
        "plans",
        "discussion",
        "deprecated",
        "resources",
        "reviews",
        "demos",
        "wrapper",
    ] {
        assert_eq!(
            encoded["example-tool"].get(field),
            Some(&serde_json::Value::Null)
        );
    }
    Ok(())
}

#[test]
fn catalog_preserves_input_order_and_omits_empty_sections() -> Result<()> {
    let rust = tag("rust", Type::Language);
    let python = tag("python", Type::Language);
    let regular = tag("regular", Type::Other);
    let mut inclusive = tag("inclusive", Type::Other);
    inclusive.include_multi = true;
    let tags = [
        rust.clone(),
        python.clone(),
        regular.clone(),
        inclusive.clone(),
    ];
    let mut raw = parsed()?;
    raw.tags = BTreeSet::from(["rust".into(), "regular".into(), "inclusive".into()]).try_into()?;
    raw.name = String::from("Z Single").try_into()?;
    let single = Entry::from_parsed(raw.clone(), &tags)?;
    raw.tags = raw
        .tags
        .iter()
        .cloned()
        .chain(["python".into()])
        .collect::<BTreeSet<_>>()
        .try_into()?;
    raw.name = String::from("A Multi").try_into()?;
    let multi = Entry::from_parsed(raw, &tags)?;
    let tools = [single.clone(), multi.clone()];
    let catalog = create_catalog(
        &tools,
        &[python, rust.clone()],
        &[
            regular.clone(),
            inclusive.clone(),
            tag("absent", Type::Other),
        ],
        vec![],
    );
    assert_eq!(catalog.multi, [multi]);
    assert_eq!(catalog.linters.len(), 1);
    assert_eq!(catalog.linters[&rust], std::slice::from_ref(&single));
    assert_eq!(catalog.others.len(), 2);
    assert_eq!(catalog.others[&regular], [single]);
    assert_eq!(catalog.others[&inclusive], tools);
    Ok(())
}

#[test]
fn catalog_rows_keep_column_major_layout_for_partial_rows() {
    for (count, expected) in [
        (0, vec![]),
        (1, vec![vec!["0"]]),
        (2, vec![vec!["0", "1"]]),
        (3, vec![vec!["0", "1", "2"]]),
        (4, vec![vec!["0", "2"], vec!["1", "3"]]),
        (5, vec![vec!["0", "2", "4"], vec!["1", "3"]]),
        (6, vec![vec!["0", "2", "4"], vec!["1", "3", "5"]]),
        (7, vec![vec!["0", "3", "6"], vec!["1", "4"], vec!["2", "5"]]),
    ] {
        let map: BTreeMap<_, _> = (0..count)
            .map(|i| (tag(&i.to_string(), Type::Language), vec![]))
            .collect();
        let catalog = Catalog {
            linters: map.clone(),
            others: map,
            multi: vec![],
            collections: vec![],
        };
        for rows in [catalog.linter_rows(), catalog.other_rows()] {
            let names: Vec<Vec<_>> = rows
                .into_iter()
                .map(|row| row.into_iter().map(|(tag, _)| tag.value.as_str()).collect())
                .collect();
            assert_eq!(names, expected);
        }
    }
}

#[test]
fn stats_keep_last_duplicate_and_strip_repeated_leading_prefixes_only() {
    let raw = stats::StatsRaw {
        data: stats::Data {
            result: [
                ("/tool/example", "1"),
                ("/tool/example", "2"),
                ("/tool//tool/repeated", "3"),
                ("/other/tool/path", "4"),
                ("/tool/", "5"),
            ]
            .into_iter()
            .map(|(path, value)| stats::Result {
                metric: stats::Metric { path: path.into() },
                value: (0.0, value.into()),
            })
            .collect(),
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(
        format_stats(raw),
        BTreeMap::from([
            ("example".into(), "2".into()),
            ("repeated".into(), "3".into()),
            ("/other/tool/path".into(), "4".into()),
            (String::new(), "5".into()),
        ])
    );
}
