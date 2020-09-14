use askama::Template;
use render::types::{Entry, Tags};
use render::{group, validate};
use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::io;
fn get_files() -> Result<(String, String), Box<dyn Error>> {
    let files: Vec<_> = env::args().skip(1).collect();
    if files.len() != 2 {
        return Err("Expected two input parameters: `tools` directory and `tags.yml path`".into());
    }
    Ok((files[0].clone(), files[1].clone()))
}

fn read_tags(file: String) -> Result<Tags, Box<dyn Error>> {
    let f = std::fs::File::open(file)?;
    Ok(serde_yaml::from_reader(f)?)
}

fn read_tools(file: String) -> Result<Vec<Entry>, Box<dyn Error>> {
    let dir: std::fs::ReadDir = std::fs::read_dir(file)?;

    let files = dir
        .map(|res| res.map(|e| e.path()))
        .filter(|x| match x {
            Ok(pb) => pb.extension().and_then(OsStr::to_str) == Some("yml"),
            Err(_) => false,
        })
        .collect::<Result<Vec<_>, io::Error>>()?;

    files
        .iter()
        .map(|p| {
            let file = std::fs::File::open(p)?;
            dbg!(&p);
            let entry: Entry = serde_yaml::from_reader(file)?;
            Ok(entry)
        })
        .collect::<Result<Vec<Entry>, _>>()

    // files.map(|x| {
    //     // println!("{:?}", f)
    //     match x {
    //         Ok(f) => {
    //             let file = std::fs::File::open(f);
    //             serde_yaml::from_reader(file)
    //         }
    //     }
    // });
    // .collect()?;

    // let f = std::fs::File::open(file)?;
    // Ok(serde_yaml::from_reader(f)?)
    // Ok(serde_yaml::from_str("{}")?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (tags, tools) = get_files()?;
    let tags = read_tags(tags)?;
    let mut tools = read_tools(tools)?;
    tools.sort();
    validate(&tags, &tools)?;

    let catalog = group(&tags, tools)?;
    println!("{}", catalog.render()?);
    Ok(())
}
