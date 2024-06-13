use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // first value in args is the name of the binary
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = match args.next() {
            Some(arg) => match arg.as_str() {
                "true" => true,
                "t" => true,
                "false" => false,
                "f" => false,
                _ => false,
            },
            None => env::var("IGNORE_CASE").is_ok(),
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search(&config.query, &file_contents)
    };

    for result in results {
        println!("{result}");
    }

    Ok(())
}

pub fn search_case_insensitive<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    search_with_closure(file_contents, |line| {
        line.to_lowercase().contains(&query.to_lowercase())
    })
}

pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    search_with_closure(file_contents, |line| line.contains(query))
}

fn search_with_closure<'a>(
    file_contents: &'a str,
    predicate: impl FnMut(&&str) -> bool,
) -> Vec<&'a str> {
    file_contents.lines().filter(predicate).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
