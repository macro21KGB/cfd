// read a file taken in as a command line argument
// and print it to stdout
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process;

fn clean_function_declaration(str: String, re_white_spaces: &Regex) -> String {
    let mut new_line = String::from(re_white_spaces.replace_all(&str, " "));

    new_line = new_line.replace("override", "").replace(";", " {}");
    return new_line;
}

struct Config {
    filename: String,
    dest_file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let dest_file_name = args[2].clone();
        Ok(Config {
            filename,
            dest_file_name,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut vec = Vec::new();

    // Setup regex
    let re = Regex::new("(.*\\(.*\\) ?[a-zA-Z_:{}&,* ]*;)").unwrap();
    let re_white_spaces = Regex::new(r"\s+").unwrap();

    let file = File::open(config.filename).unwrap();
    let mut dest_file = File::options()
        .write(true)
        .read(false)
        .append(true)
        .open(config.dest_file_name)
        .unwrap();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();

        if re.is_match(&line) {
            vec.push(clean_function_declaration(line, &re_white_spaces));
        }
    }

    dest_file
        .write_all(vec.join("\n\n").as_bytes())
        .expect("Errore nella Scrittura");
}
