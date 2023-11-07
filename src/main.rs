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
            let refined_args = Vec::from([args[2].clone(), args[3].clone()]);
            let config = SearchForFileConfig::build_search_for_file_config(&refined_args);
            search_file_parent_helper(config);
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

pub fn search_file_parent_helper(config: SearchForFileConfig) {
    let mut dir_queue: Vec<String> = Vec::new();

    dir_queue.push(config.parent_location.to_string());
    while dir_queue.len() > 0 {
        if search_for_file_by_queue(config.file_name, &mut dir_queue) {
            println!("found file");
            break;
        }
    }
}

pub fn search_for_file_by_queue(file_name_from_user: &str, dir_queue: &mut Vec<String>) -> bool {
    let curr_dir = dir_queue.pop().unwrap();
    println!("\ncurrently in {} dir \n", curr_dir);
    let mut args: Vec<String> = Vec::new();

    let clone = curr_dir.clone();
    args.push(file_name_from_user.to_string());

    args.push(curr_dir.clone());

    let search_file_config = SearchForFileConfig::build_search_for_file_config(&args);

    let result = fs::read_dir(search_file_config.parent_location).expect("message");
    for file in result {
        let entry = file.unwrap();
        let path = entry.path();
        let file_name = entry.file_name().to_str().unwrap().to_string();
        let file_type = entry.file_type(); // Get the file type
        let name_clone = file_name.clone();
        if let Err(_) = file_type {
            panic!("unknown type");
        }
        if path.is_file() {
            let name = name_clone.clone();
            if *name == *search_file_config.file_name {
                return true;
            }
        } else if path.is_dir() {
            dir_queue.push(clone.clone() + &path.to_string_lossy().to_string());
        } else {
            println!("Unknown: {}", file_name);
        }
    }
    return false;
}
