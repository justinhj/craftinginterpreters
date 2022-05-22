use rlox::eval::eval_statements;
use rlox::parse::parse;
use rlox::scan::scan;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "s", long)]
    show_scan: bool,

    #[structopt(short = "p", long)]
    show_parse: bool,

    #[structopt(parse(from_os_str))]
    inputfile: Option<PathBuf>,
}

fn main() {
    let Opt {
        show_scan,
        show_parse,
        inputfile,
    } = Opt::from_args();

    match inputfile {
        Some(f) => {
            let source = fs::read_to_string(f).unwrap();
            match scan(&source) {
                Ok(tokens) => {
                    if show_scan {
                        println!("Tokens:");
                        tokens.iter().for_each(|token| println!("\t{:?}", token));
                    }
                    match parse(&tokens) {
                        Ok(parsed) => {
                            if show_parse {
                                println!("\nParsed AST:\n");
                                for statement in &parsed {
                                    println!("\t{}", statement)
                                }
                            }
                            match eval_statements(&parsed) {
                                Ok(_) => println!("Done"),
                                Err(err) => println!("Error: {:?}", err),
                            }
                        }
                        Err(err) => {
                            println!("{:?}", err)
                        }
                    }
                }
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
                        Ok(tokens) => {
                            if show_scan {
                                println!("Tokens:");
                                tokens.iter().for_each(|token| println!("\t{:?}", token));
                            }
                            match parse(&tokens) {
                                Ok(parsed) => {
                                    rl.add_history_entry(line.as_str());
                                    if show_parse {
                                        println!("\nParsed AST:\n\n")
                                    }
                                    let eval_result = eval_statements(&parsed);
                                    println!("Eval result: {:?}", eval_result);
                                }
                                Err(err) => {
                                    println!("{:?}", err)
                                }
                            }
                        }
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
