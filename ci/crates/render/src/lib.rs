use slug::slugify;
use stats::StatsRaw;
use std::collections::BTreeMap;
use types::{Api, ApiEntry, Catalog, Collection, Entry, Tag, Type};

mod deprecation;
pub mod stats;
pub mod types;
mod validated;

pub use deprecation::check_deprecated;

/// Groups normalized entries for the generated README.
#[must_use]
pub fn create_catalog(
    entries: &[Entry],
    languages: &[Tag],
    other_tags: &[Tag],
    collections: Vec<Collection>,
) -> Catalog {
    // Multi-language tools get their own primary section instead of being repeated under
    // every language. They still belong in applicable non-language tag sections.
    let (multi, single_language): (Vec<&Entry>, Vec<&Entry>) = entries.iter().partition(|entry| {
        let language_tags = entry
            .tags
            .iter()
            .filter(|t| t.kind == Type::Language)
            .count();
        language_tags > 1 && !entry.is_c_cpp()
    });

    let mut linters = BTreeMap::new();
    for language in languages {
        let list: Vec<Entry> = single_language
            .iter()
            .copied()
            .filter(|e| e.tags.contains(language))
            .cloned()
            .collect();
        if !list.is_empty() {
            linters.insert(language.clone(), list);
        }
    }

    let mut others = BTreeMap::new();
    for other in other_tags {
        let list: Vec<Entry> = if other.include_multi {
            entries
                .iter()
                .filter(|e| e.tags.contains(other))
                .cloned()
                .collect()
        } else {
            single_language
                .iter()
                .copied()
                .filter(|e| e.tags.contains(other))
                .cloned()
                .collect()
        };
        if !list.is_empty() {
            others.insert(other.clone(), list);
        }
    }

    Catalog {
        linters,
        others,
        multi: multi.into_iter().cloned().collect(),
        collections,
    }
}

/// Converts normalized entries to the machine-readable API representation.
#[must_use]
pub fn create_api(entries: Vec<Entry>, languages: &[Tag], other_tags: &[Tag]) -> Api {
    let mut api_entries = BTreeMap::new();

    for entry in entries {
        // Preserve configured tag order rather than the entry's set order.
        let entry_languages = languages
            .iter()
            .filter(|lang| entry.tags.contains(lang))
            .map(|lang| lang.value.clone())
            .collect();

        let entry_other = other_tags
            .iter()
            .filter(|other| entry.tags.contains(other))
            .map(|other| other.value.clone())
            .collect();

        let key = slugify(&entry.name);
        let api_entry = ApiEntry {
            name: entry.name.into(),
            categories: entry.categories,
            languages: entry_languages,
            other: entry_other,
            // Compound license strings remain a single API value.
            licenses: vec![entry.license],
            types: entry.types,
            homepage: entry.homepage,
            source: entry.source,
            pricing: entry.pricing,
            plans: entry.plans,
            description: entry.description,
            discussion: entry.discussion,
            deprecated: entry.deprecated,
            resources: entry.resources,
            reviews: entry.reviews,
            demos: entry.demos,
            wrapper: entry.wrapper,
        };
        api_entries.insert(key, api_entry);
    }

    api_entries
}

/// Converts raw page-view statistics into a tool-name lookup.
#[must_use]
pub fn format_stats(stats: StatsRaw) -> BTreeMap<String, String> {
    stats
        .data
        .result
        .into_iter()
        .map(|result| {
            (
                result.metric.path.trim_start_matches("/tool/").to_string(),
                result.value.1,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};
    use askama::Template;
    use std::collections::BTreeSet;

    fn tag(name: &str, value: &str, kind: Type) -> Tag {
        Tag {
            name: name.into(),
            value: value.into(),
            kind,
            include_multi: false,
        }
    }

    fn entry(tags: &[Tag]) -> Result<Entry> {
        Ok(Entry {
            name: String::from("Multi Tool").try_into()?,
            categories: BTreeSet::new(),
            tags: tags.iter().cloned().collect::<BTreeSet<_>>().try_into()?,
            license: "MIT".into(),
            types: BTreeSet::new(),
            homepage: "https://example.com".into(),
            source: None,
            pricing: None,
            plans: None,
            description: "Example tool".into(),
            discussion: None,
            deprecated: None,
            resources: None,
            reviews: None,
            demos: None,
            wrapper: None,
        })
    }

    #[test]
    fn deprecated_tools_are_collapsed_in_every_section() -> Result<()> {
        let tags = [tag("Rust", "rust", Type::Language)];
        let mut active = entry(&tags)?;
        active.name = String::from("Active Tool").try_into()?;
        let mut deprecated = entry(&tags)?;
        deprecated.name = String::from("Deprecated Tool").try_into()?;
        deprecated.deprecated = Some(true);
        deprecated.license = "proprietary".into();
        deprecated.discussion = Some("https://example.com/discussion".into());
        let tools = vec![deprecated, active];
        let catalog = Catalog {
            linters: BTreeMap::from([(tag("Rust", "rust", Type::Language), tools.clone())]),
            others: BTreeMap::from([(tag("Security", "security", Type::Other), tools.clone())]),
            multi: tools,
            collections: vec![],
        };
        let markdown = catalog.render()?;

        assert_eq!(
            markdown
                .matches("<summary>Show Deprecated</summary>")
                .count(),
            3
        );
        assert_eq!(
            markdown
                .matches("[Active Tool](https://example.com)")
                .count(),
            3
        );
        assert_eq!(markdown.matches("**Deprecated Tool**").count(), 3);
        assert!(!markdown.contains("[Deprecated Tool]("));
        assert!(!markdown.contains("<details open"));
        for heading in [
            "<h2>Rust</h2>",
            "## Multiple languages",
            "<h2>Security</h2>",
        ] {
            let section = markdown.split_once(heading).context("Missing section")?.1;
            let active_position = section
                .find("[Active Tool]")
                .context("Missing active tool")?;
            let details_position = section.find("<details>").context("Missing details")?;
            assert!(active_position < details_position);
            let hidden = section[details_position..]
                .split_once("</details>")
                .context("Unclosed details")?
                .0;
            assert!(hidden.contains("<summary>Show Deprecated</summary>\n\n- **Deprecated Tool**"));
            assert!(hidden.contains(
                "[:information_source:](<https://example.com/discussion>) :warning: :copyright:"
            ));
            assert!(!hidden.contains("[Active Tool]"));
        }
        Ok(())
    }

    #[test]
    fn no_empty_deprecated_sections_are_rendered() -> Result<()> {
        let tags = [tag("Rust", "rust", Type::Language)];
        let mut explicitly_active = entry(&tags)?;
        explicitly_active.deprecated = Some(false);
        for tools in [vec![], vec![entry(&tags)?], vec![explicitly_active]] {
            let markdown = Catalog {
                linters: BTreeMap::new(),
                others: BTreeMap::new(),
                multi: tools,
                collections: vec![],
            }
            .render()?;
            assert!(!markdown.contains("Show Deprecated"));
        }
        Ok(())
    }

    #[test]
    fn deprecated_only_sections_keep_their_entries() -> Result<()> {
        let mut tool = entry(&[tag("Rust", "rust", Type::Language)])?;
        tool.deprecated = Some(true);
        let markdown = Catalog {
            linters: BTreeMap::new(),
            others: BTreeMap::new(),
            multi: vec![tool],
            collections: vec![],
        }
        .render()?;
        assert_eq!(
            markdown
                .matches("<summary>Show Deprecated</summary>")
                .count(),
            1
        );
        assert!(markdown.contains("\n\n- **Multi Tool** :warning: — Example tool\n\n</details>"));
        Ok(())
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("this is a test"), "this-is-a-test".to_string());
        assert_eq!(slugify("Big"), "big".to_string());
        assert_eq!(slugify("   Big"), "big".to_string());
        assert_eq!(slugify("Astrée"), "astree".to_string());
        assert_eq!(slugify("non word 1234"), "non-word-1234".to_string());
        assert_eq!(slugify("it-has-dashes"), "it-has-dashes".to_string());
        assert_eq!(
            slugify("   - - it-has-dashes - -"),
            "it-has-dashes".to_string()
        );
    }

    #[test]
    fn multi_language_tools_remain_visible_in_other_sections_and_api() -> Result<()> {
        let python = tag("Python", "python", Type::Language);
        let rust = tag("Rust", "rust", Type::Language);
        let mut ai_generated = tag("AI-generated code", "ai-generated-code", Type::Other);
        ai_generated.include_multi = true;
        let tool = entry(&[python.clone(), rust.clone(), ai_generated.clone()])?;
        let languages = [python, rust];
        let other_tags = [ai_generated.clone()];

        let catalog = create_catalog(std::slice::from_ref(&tool), &languages, &other_tags, vec![]);

        assert!(catalog.linters.is_empty());
        assert_eq!(catalog.multi.len(), 1);
        assert_eq!(catalog.multi[0], tool);
        assert_eq!(catalog.others[&ai_generated].len(), 1);
        assert_eq!(catalog.others[&ai_generated][0], tool);

        let api = create_api(vec![tool], &languages, &other_tags);
        assert_eq!(api["multi-tool"].languages, ["python", "rust"]);
        assert_eq!(api["multi-tool"].other, ["ai-generated-code"]);
        Ok(())
    }

    #[test]
    fn c_and_cpp_tools_stay_in_language_sections_when_they_have_other_tags() -> Result<()> {
        let c = tag("C", "c", Type::Language);
        let cpp = tag("C++", "cpp", Type::Language);
        let security = tag("Security/SAST", "security", Type::Other);
        let tool = entry(&[c.clone(), cpp.clone(), security.clone()])?;

        let catalog = create_catalog(
            std::slice::from_ref(&tool),
            &[c.clone(), cpp.clone()],
            std::slice::from_ref(&security),
            vec![],
        );

        assert!(catalog.multi.is_empty());
        assert_eq!(catalog.linters[&c].len(), 1);
        assert_eq!(catalog.linters[&c][0], tool);
        assert_eq!(catalog.linters[&cpp].len(), 1);
        assert_eq!(catalog.linters[&cpp][0], tool);
        assert_eq!(catalog.others[&security].len(), 1);
        assert_eq!(catalog.others[&security][0], tool);
        Ok(())
    }
}
