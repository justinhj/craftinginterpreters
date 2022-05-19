use rlox::parse::parse;
use rlox::scan::scan;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short, long)]
    scan_only: bool,

    #[structopt(short, long)]
    parse_only: bool,

    #[structopt(parse(from_os_str))]
    inputfile: Option<PathBuf>,
}

fn main() {
    let Opt {
        scan_only,
        parse_only,
        inputfile,
    } = Opt::from_args();

    match inputfile {
        Some(f) => {
            let source = fs::read_to_string(f).unwrap();
            match scan(&source) {
                Ok(tokens) => {
                    println!("Tokens:");
                    tokens.iter().for_each(|token| println!("\t{:?}", token));
                    if !scan_only {
                        match parse(&tokens) {
                        Ok(parsed) => {
                            println!("\nParsed AST:\n\t{}", parsed)
                        }
                        Err(err) => {
                            println!("{:?}", err)
                        }
                    }
                }},
                Err(err) => println!("Error {:?}", err),
            }
        }
        None => {
            // `()` can be used when no completer is required
            let mut rl = Editor::<()>::new();
            println!("Lox scanner");
            if rl.load_history("history.txt").is_err() {
                println!("No previous history.");
            }
            loop {
                let readline = rl.readline(">> ");
                match readline {
                    Ok(line) => match scan(&line) {
                        Ok(tokens) => match parse(&tokens) {
                            Ok(parsed) => {
                                rl.add_history_entry(line.as_str());
                                tokens.iter().for_each(|token| println!("{:?}", token));
                                println!("Parsed {}", parsed)
                            }
                            Err(err) => {
                                println!("{:?}", err)
                            }
                        },
                        Err(err) => println!("Error {:?}", err),
                    },
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }
            rl.save_history("history.txt").unwrap();
        }
    }
}
