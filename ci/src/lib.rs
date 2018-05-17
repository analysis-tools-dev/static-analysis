// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

extern crate regex;

use failure::{Error, err_msg};
use regex::Regex;
use std::fmt;
use std::cmp::Ordering;

lazy_static! {
    static ref TOOL_REGEX: Regex = Regex::new(r"\*\s\[(?P<name>.*)\]\((?P<link>http[s]?://.*)\)\s(:copyright:\s)?\-\s(?P<desc>.*)").unwrap();
    static ref SUBSECTION_HEADLINE_REGEX: Regex = Regex::new(r"[A-Za-z\s]*").unwrap();
}

struct Tool {
    name: String,
    link: String,
    desc: String,
}

impl Tool {
    fn new<T: Into<String>>(name: T, link: T, desc: T) -> Self {
        Tool {
            name: name.into(),
            link: link.into(),
            desc: desc.into(),
        }
    }
}

impl PartialEq for Tool {
    fn eq(&self, other: &Tool) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl Eq for Tool {}

impl PartialOrd for Tool {
    fn partial_cmp(&self, other: &Tool) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tool {
    fn cmp(&self, other: &Tool) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

fn check_tool(tool: &str) -> Result<Tool, Error> {
    println!("Checking `{}`", tool);
    // NoneError can not implement Fail at this time. That's why we use ok_or
    // See https://github.com/rust-lang-nursery/failure/issues/61
    let captures = TOOL_REGEX.captures(tool).ok_or(err_msg(format!("Tool does not match regex: {}", tool)))?;

    let name = captures["name"].to_string();
    let link = captures["link"].to_string();
    let desc = captures["desc"].to_string();

    if name.len() > 50 {
        bail!("Name of tool is suspiciously long: `{}`", name);
    }

    // A somewhat arbitrarily chosen description length.
    // Note that this includes any markdown formatting
    // like links. Therefore we are quite generous for now.
    if desc.len() > 200 {
        bail!("Desription of `{}` is too long: {}", name, desc);
    }

    Ok(Tool::new(name, link, desc))
}

fn check_section(section: String) -> Result<(), Error> {
    // Ignore license section
    if section.starts_with("License") {
        return Ok(());
    }

    // Skip section headline
    let lines: Vec<_> = section.split('\n').skip(1).collect();
    if lines.is_empty() {
        bail!("Empty section: {}", section)
    };

    let mut tools = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }
        // Exception for subsection headlines
        if !line.starts_with("*") && line.ends_with(":") &&
            SUBSECTION_HEADLINE_REGEX.is_match(line)
        {
            continue;
        }
        tools.push(check_tool(line)?);
    }
    // Tools need to be alphabetically ordered
    check_ordering(tools)
}

fn check_ordering(tools: Vec<Tool>) -> Result<(), Error> {
    match tools.windows(2).find(|t| t[0] > t[1]) {
        Some(tools) => bail!("`{}` does not conform to alphabetical ordering", tools[0].name),
        None => Ok(()),
    }
}

fn check(text: String) -> Result<(), Error> {
    let sections = text.split("\n# ");

    // Skip first two sections,
    // as they contain the prelude and the table of contents.
    for section in sections.skip(2) {
        let subsections = section.split("## ");
        for subsection in subsections.skip(1) {
            check_section(subsection.into())?;
        }
    }
    Ok(())
}

mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_complete_file() {
        let mut file = File::open("../README.md").expect("Can't open testfile");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Can't read testfile contents");
        let result = check(contents);
        if result.is_err() {
            println!("Error: {:?}", result.err());
            assert!(false);
        } else {
            assert!(true);
        }
    }


    #[test]
    fn test_correct_ordering() {
        assert!(check_ordering(vec![]).is_ok());

        assert!(check_ordering(vec![Tool::new("a", "url", "desc")]).is_ok());

        assert!(
            check_ordering(vec![
                Tool::new("0", "", ""),
                Tool::new("1", "", ""),
                Tool::new("a", "", ""),
                Tool::new("Axx", "", ""),
                Tool::new("B", "", ""),
                Tool::new("b", "", ""),
                Tool::new("c", "", ""),
            ]).is_ok()
        );
    }

    #[test]
    fn test_incorrect_ordering() {
        assert!(
            check_ordering(vec![
                Tool::new("b", "", ""),
                Tool::new("a", "", ""),
                Tool::new("c", "", ""),
            ]).is_err()
        );
    }
}
