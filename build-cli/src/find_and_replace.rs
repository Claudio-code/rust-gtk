use regex::Regex;
use text_colorizer::*;
use std::{env, fs};

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!("{} - replace a string with a new string", "Find and replace".green());
    eprintln!("Usage: <target string> <replace string> <INPUT FILE> <OUTPUT FILE>");
}

fn replace(target: &str, rep: &str, data: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, rep).to_string())
}

fn read_file(args: &Arguments) -> String {
    match fs::read_to_string(&args.input_file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{} failed to read from file {}: {:?}", "Error".red().bold(), args.input_file, e);
            std::process::exit(1);
        }
    }
}

fn write_file(output_file: &String, data: &String) {
    match fs::write(&output_file, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} failed to write from file {}: {:?}", "Error".red().bold(), output_file, e);
            std::process::exit(1);
        }
    };
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!("{} wrong number of arguments give. Expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        pattern: args[0].clone(),
        replace: args[1].clone(),
        input_file: args[2].clone(),
        output_file: args[3].clone()
    }
}

pub fn run() {
    let args = parse_args();
    let data = read_file(&args);
    let replace_data = match replace(&args.pattern, &args.replace, &data) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} failed to replace text: {:?}", "Error".red().bold(), e);
            std::process::exit(1);
        }
    };
    write_file(&args.output_file, &replace_data);
}
