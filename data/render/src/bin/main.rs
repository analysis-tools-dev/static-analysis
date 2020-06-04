use render::types::{Entry, Tags};
use render::{group, validate};
use std::env;
use std::error::Error;

fn get_files() -> Result<(String, String), Box<dyn Error>> {
    let files: Vec<_> = env::args().skip(1).collect();
    if files.len() != 2 {
        return Err("Expected a two input files, `tools.yml` and `tags.yml`".into());
    }
    Ok((files[0].clone(), files[1].clone()))
}

fn read_categories(file: String) -> Result<Tags, Box<dyn Error>> {
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
    println!("{}", catalog.render());
    Ok(())
}
