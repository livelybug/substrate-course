use std::fs;
// use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Config {
    pub keyword: String,
    pub ignore_case: bool,
    pub input_filename: String,
    pub output_filename: String,
    pub match_count: bool
}



pub fn run(config:Config) -> Vec<String>{
    let contents = fs::read_to_string(&config.input_filename).unwrap();

    let res = search_(&config, &contents);
    res
}

pub fn search_<'a>(config:&Config, contents:&'a str) -> Vec<String> {
    let mut keyword = config.keyword.clone();
    if * &config.ignore_case == true {
        keyword = keyword.to_lowercase();
    }

    let mut res: Vec<String> = Vec::new();

    let path = Path::new(&config.output_filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    for line in contents.lines() {
        let mut _line: String = String::from(line);
        if * &config.ignore_case == true {
            _line = String::from(line.to_lowercase());
        }

        if _line.contains(&keyword) {
            if * &config.match_count == false {
                match file.write_all(format!("{}\n", line).as_bytes()) {
                    Err(why) => panic!("couldn't write to {}: {}", display, why),
                    Ok(_) => {},
                }

                res.push(line.into());
            } else {
                let v: Vec<String> = (&_line).matches(&keyword).map(|x| String::from(x)).collect();
                res.extend(v);
            }
        }
    }
    res
}
