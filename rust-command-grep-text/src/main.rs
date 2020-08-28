use std::path::Path;

#[macro_use]
extern crate clap;
use minigrep::Config;
use clap::{Arg, App, SubCommand, ArgMatches};

pub fn match_param(matches: &ArgMatches) -> Config {
    let output_filename;
    if let Some(o) = matches.value_of("output_filename") {
        output_filename = String::from(o);
        // println!("Value for output_filename: {}", o);
    } else {
        output_filename = String::from("output.txt");
    }

    let input_filename;
    if let Some(f) = matches.value_of("input_filename") {
        input_filename = String::from(f);
        // println!("Value for input_filename: {}", f);
    } else if Path::new("./input.txt").exists() {
        input_filename = String::from("input.txt");
        // println!("Use default input file: {}", input_filename);
    } else {
        panic!("No input file found!");
    }

    let keyword;
    if let Some(k) = matches.value_of("keyword") {
        keyword = String::from(k);
        // println!("Keyword for search: {}", k);
    } else {
        panic!("No keyword!");
    }

    let ignore_case;
    if matches.is_present("ignore_case") {
        // println!("Searching with case insensitive...");
        ignore_case = true;
    } else {
        ignore_case = false;
    }

    Config{keyword, ignore_case, input_filename, output_filename, match_count: false}
}

pub struct CommnadStruct<'a> {
    pub name: String,
    pub help: &'a str,
    pub arg_input: &'a Arg<'a, 'a>,
    pub arg_output:&'a Arg<'a, 'a>,
    pub arg_ignore_case:&'a Arg<'a, 'a>,
    pub arg_keyword:&'a Arg<'a, 'a>
}

pub fn compose_command<'a>(command: CommnadStruct<'a>) -> App<'a, 'a> {
    App::new(&command.name)
        .help(command.help)
        .arg(command.arg_input,)
        .arg(command.arg_output,)
        .arg(command.arg_ignore_case,)
        .arg(command.arg_keyword,)
}

fn main() {
    let arg_input = &Arg::with_name("input_filename")
                        .short("f")
                        .long("input_filename")
                        .value_name("FILE")
                        .help("Set an input file to search in, if absent, look for file named 'input.txt'")
                        .takes_value(true);

    let arg_output = &Arg::with_name("output_filename")
                        .short("o")
                        .value_name("FILE")
                        .help("Sets the output file, if absent, set output file as 'output.txt'")
                        .takes_value(true);

    let arg_ignore_case = &Arg::with_name("ignore_case")
                            .short("i")
                            .help("Ignore case when searching");

    let arg_keyword = &Arg::with_name("keyword")
                            .short("k")
                            .help("Sets the keyword to search")
                            .takes_value(true);

    let main_command = compose_command(CommnadStruct{name: "MyGrep".to_string(), help: "Grep awesome things", arg_input, arg_output, arg_keyword, arg_ignore_case});
    let sub_command_lines = compose_command(CommnadStruct{name: "lines".to_string(), help: "Number of lines matches the searching criteria", arg_input, arg_output, arg_keyword, arg_ignore_case});
    let sub_command_matches = compose_command(CommnadStruct{name: "matches".to_string(), help: "Number of matches the searching criteria", arg_input, arg_output, arg_keyword, arg_ignore_case});

    let matches = main_command
        .version("1.0")
        .author("<burtchensu@gmail.com>")
        .subcommand(sub_command_lines)
        .subcommand(sub_command_matches)
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("lines") {
        let config = match_param(&matches);
        let res = minigrep::run(config);
        println!("Number of lines found: {:?}", &res.len());
    } else if let Some(ref matches) = matches.subcommand_matches("matches") {
        let mut config = match_param(&matches);
        config.match_count = true;
        let res = minigrep::run(config);
        println!("Number of matches found: {:?}", &res.len());
    } else {
        let config = match_param(&matches);
        let res = minigrep::run(config);
        println!("{:?}", &res);
    }

}
