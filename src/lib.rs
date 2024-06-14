use std::collections::BTreeMap;
use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // fn new(args: &[String]) -> Config {
    //     if args.len() < 3 {
    //         panic!("Too less arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Config { query, file_path }
    // }
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn search<'a>(query: &'a str, content: &'a str) -> BTreeMap<i32, &'a str> {
    let mut i: i32 = 0;
    let mut results: BTreeMap<i32, &str> =  BTreeMap::new();
    for line in content.lines() {
        i+=1;
        if line.contains(query) {
            results.insert(i, line);
        }
    } 
    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for (n, line) in search(&config.query, &contents) {
        println!("{}. {}", n, line);
    }

    Ok(())
}