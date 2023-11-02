use rust_cli_minigrep::{search, search_case_insensative, Config};
use std::error::Error;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build_config(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.file_name)?;
    println!("\nResults : \n");
    if config.ignore_case {
        println!("Note: Case insensative\n");
        for line in search_case_insensative(&config.query, &file_content) {
            println!("{line}");
        }
    } else {
        println!("Note: Case sensative\n");
        for line in search(&config.query, &file_content) {
            println!("{line}");
        }
    }
    Ok(())
}
