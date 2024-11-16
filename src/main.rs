#[allow(unused_imports)]
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::{env, fs, process};

mod flags;
use crate::flags::*;

mod lexer;
use crate::lexer::*;

// Thread-safe static FLAGS using `once_cell`
static FLAGS: Lazy<Vec<Mutex<Flag>>> = Lazy::new(|| {
    vec![
        Mutex::new(Flag::new(
            "DEBUG",
            "Debugs the program | Developer use only",
            &["--debug", "-d"],
            false,
        )),
        Mutex::new(Flag::new(
            "COMPILE",
            "Compiles the program",
            &["--compile", "-c"],
            false,
        )),
        Mutex::new(Flag::new(
            "HELP",
            "Prints this message!",
            &["--help", "-h"],
            false,
        )),
    ]
});

fn usage(path: &str, err: Option<&str>) {
    println!("Usage: {} <flags> <path-to-file>", path);
    println!("Flags:");

    // Clone the flags to avoid deadlocks
    let flags_snapshot: Vec<Flag> = FLAGS
        .iter()
        .map(|flag| flag.lock().unwrap().clone())
        .collect();

    print_flags(&flags_snapshot);

    if let Some(message) = err {
        println!("Error: {}", message);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        usage(
            args.first().unwrap(),
            Some("Did not provide enough arguments"),
        );
        process::exit(1);
    }

    let file_path: String = args.last().unwrap().to_string();
    let contents: String = fs::read_to_string(file_path).expect("Expected to read file");

    // Loop through the arguments
    for arg in args.iter().skip(1) {
        if arg == args.last().unwrap() {
            continue;
        }
        for flag in FLAGS.iter() {
            let mut flag = flag.lock().unwrap();
            if flag.is_in(arg) {
                flag.default = true;
            }
        }
    }

    let rules = lexer_rules();

    let lexer = santiago::lexer::lex(&rules, &contents);
    let grammar = grammar();

    FLAGS
        .iter()
        .filter_map(|flag| {
            let flag = flag.lock().unwrap();
            if flag.default {
                Some(flag)
            } else {
                None
            }
        })
        .for_each(|flag| match flag.name {
            "HELP" => {
                usage(args.first().unwrap(), None);
                process::exit(0);
            }
            "DEBUG" => {
                println!("-------------------------------");
                println!("Debug mode | Developer use only");
                println!("-------------------------------");
                match &lexer {
                    Ok(lexemes) => {
                        println!("Lexemes:");
                        for lexeme in lexemes {
                            println!("    {lexeme}");
                        }

                        match santiago::parser::parse(&grammar, &lexemes) {
                            Ok(parse_trees) => {
                                println!("Parse Trees:");
                                for ast in parse_trees {
                                    println!("{ast}");
                                }
                            }
                            Err(error) => {
                                println!("Parsing Error:");
                                println!("{error}");
                            }
                        }
                    }
                    Err(error) => {
                        println!("Parsing Error:");
                        println!("{error}");
                    }
                }
            }
            "COMPILE" => {
                println!("Compile flag is set. Starting compilation...");
            }
            _ => println!("Unknown flag: {}", flag.name),
        });
}
