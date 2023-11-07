use std::env;
pub struct Config {
    pub file_name: String,
    pub query: String,
    pub ignore_case: bool,
}
impl Config {
    pub fn build_config(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let file_name = args[2].clone();
        let query = args[3].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            file_name,
            query,
            ignore_case,
        })
    }
}
pub struct SearchForFileConfig<'a> {
    pub file_name: &'a str,
    pub parent_location: &'a str,
}

impl<'a> SearchForFileConfig<'a> {
    pub fn build_search_for_file_config(args: &[String]) -> SearchForFileConfig {
        let file_name = &args[0];
        let parent_location = &args[1];

        SearchForFileConfig {
            file_name,
            parent_location,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensative() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensative() {
        let query = "RusT";
        let contents = "\
Rust:
safe, fast, productive
Pick there
rust
        ";
        assert_eq!(
            vec!["Rust:", "rust"],
            search_case_insensative(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

pub fn search_case_insensative<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}
