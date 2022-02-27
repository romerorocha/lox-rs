use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

type ScanResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    filename: String,
}

pub fn get_args() -> ScanResult<Config> {
    let matches = App::new("lox")
        .version("0.1.0")
        .author("Claudio Romero")
        .about("Rust implementation of Lox Language")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("Input file")
                .default_value("-"),
        )
        .get_matches();

    Ok(Config {
        filename: matches.value_of("file").unwrap().to_string(),
    })
}

pub fn run_file(config: Config) -> ScanResult<()> {
    match open(&config.filename) {
        Err(err) => eprintln!("Failed to open {}:  {}", config.filename, err),
        Ok(file) => println!("{}", config.filename),
    }

    Ok(())
}

fn open(filename: &str) -> ScanResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
