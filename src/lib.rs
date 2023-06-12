use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn run(config: Config) -> MyResult<()> {
    for path in config.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Err(e) => eprintln!("{}", e),
                Ok(entry) => println!("{}", entry.path().display()),
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .about("Rust find")
        .arg(
            Arg::with_name("paths")
                .value_name("PATH")
                .help("Search paths")
                .multiple(true)
                .default_value("."),
        )
        .arg(
            Arg::with_name("names")
                .value_name("NAME")
                .short("n")
                .long("name")
                .help("Name")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("types")
                .value_name("TYPE")
                .short("t")
                .long("type")
                .help("Entry type")
                .takes_value(true)
                .multiple(true)
                .possible_values(&["f", "d", "l"]),
        )
        .get_matches();

    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| Regex::new(&name).map_err(|_| format!("Invalid --name \"{}\"", name)))
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|val| match val.as_str() {
                    "d" => EntryType::Dir,
                    "f" => EntryType::File,
                    "l" => EntryType::Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(Config {
        paths: matches.values_of_lossy("paths").unwrap(),
        names,
        entry_types,
    })
}
