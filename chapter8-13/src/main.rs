use rlox::eval::eval_statements;
use rlox::eval::EvalState;
use rlox::parse::parse;
use rlox::scan::scan;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "s", long)]
    show_scan: bool,

    #[structopt(short = "p", long)]
    show_parse: bool,

    #[structopt(short, long, help = "When false disable evaluation")]
    eval_enabled: Option<bool>,

    #[structopt(parse(from_os_str))]
    inputfile: Option<PathBuf>,
}

fn main() {
    let Opt {
        show_scan,
        show_parse,
        inputfile,
        eval_enabled,
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
                            if eval_enabled.unwrap_or(true) {
                                let eval_state = EvalState::new();
                                match eval_statements(&parsed, Rc::new(RefCell::new(eval_state))) {
                                    Ok(_) => println!("Done"),
                                    Err(err) => println!("Error: {:?}", err),
                                }
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
                                        println!("\nParsed AST:\n\n");
                                        for statement in &parsed {
                                            println!("\t{}", statement)
                                        }
                                    }
                                    if eval_enabled.unwrap_or(true) {
                                        let eval_state = EvalState::new();
                                        let eval_result = eval_statements(
                                            &parsed,
                                            Rc::new(RefCell::new(eval_state)),
                                        );
                                        println!("Eval result: {:?}", eval_result);
                                    }
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
