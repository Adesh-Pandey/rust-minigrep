use rust_cli_minigrep::{search, search_case_insensative, Config, SearchForFileConfig};
use std::error::Error;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let control_string = &args[1];

    match control_string.as_str() {
        "-sif" => {
            search_in_file(&args);
        }
        "-s" => {
            search_for_file(&args);
        }

        _ => {
            eprintln!("Operation unknown");
        }
    }
}

fn search_in_file(args: &[String]) {
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

pub fn search_for_file(args: &[String]) {
    let search_file_config = SearchForFileConfig::build_search_for_file_config(args);

    let result = fs::read_dir(search_file_config.parent_location);

    if let Err(_) = result {
        eprintln!("Error occured while trying to read specified dir")
    } else {
        for file in result.unwrap() {
            match file {
                Ok(entry) => {
                    let path = entry.path();
                    let file_name = entry.file_name();
                    let file_type = entry.file_type(); // Get the file type

                    if let Err(_) = file_type {
                        panic!("unknown type");
                    }

                    if path.is_file() {
                        println!("File: {}", file_name.to_string_lossy());
                    } else if path.is_dir() {
                        println!("Directory: {}", file_name.to_string_lossy());
                    } else {
                        println!("Unknown: {}", file_name.to_string_lossy());
                    }
                }
                _ => (),
            }
        }
    }
}
