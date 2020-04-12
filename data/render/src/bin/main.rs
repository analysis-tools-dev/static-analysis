use render::types::{Categories, Entry};
use render::{group, render, validate};
use std::env;
use std::error::Error;

fn get_files() -> Result<(String, String), Box<dyn Error>> {
    let files: Vec<_> = env::args().skip(1).collect();
    if files.len() != 2 {
        return Err("Expected a two input files, `data.yml` and `categories.yml`".into());
    }
    Ok((files[0].clone(), files[1].clone()))
}

fn read_categories(file: String) -> Result<Categories, Box<dyn Error>> {
    let f = std::fs::File::open(file)?;
    Ok(serde_yaml::from_reader(f)?)
}

fn read_entries(file: String) -> Result<Vec<Entry>, Box<dyn Error>> {
    let f = std::fs::File::open(file)?;
    Ok(serde_yaml::from_reader(f)?)
}
fn main() -> Result<(), Box<dyn Error>> {
    let (categories, data) = get_files()?;
    let categories = read_categories(categories)?;
    let mut entries = read_entries(data)?;
    entries.sort();
    validate(&categories, &entries)?;

    let catalog = group(&categories, entries)?;
    let template = std::fs::read_to_string("src/templates/README.md")?;
    let rendered = render(&template, catalog, categories)?;
    println!("{}", rendered);
    Ok(())
}
