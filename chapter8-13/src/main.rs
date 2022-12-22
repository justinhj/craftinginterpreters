use rlox::eval::eval_statements;
use rlox::eval::EvalState;
use rlox::parse::parse;
use rlox::parse::ParseError;
use rlox::scan::scan;
use rlox::scan::ScanError;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use structopt::StructOpt;
use rlox::eval::RuntimeError;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "s", long)]
    show_scan: bool,

    #[structopt(short = "p", long)]
    show_parse: bool,

    #[structopt(short = "e", long)]
    eval_enabled: Option<bool>,

    #[structopt(parse(from_os_str))]
    inputfile: Option<PathBuf>,
}

// Note: this error handling mechanism comes from https://fettblog.eu/rust-enums-wrapping-errors/
// The idea is to make an enum that covers all application layer errors and then implement From
// trait for each "real" error into the application level one here...

#[derive(Debug)]
enum InterpreterError {
    FileNotFound(&'static str),
    ScanError(ScanError),
    ParseError(ParseError),
    RuntimeError(RuntimeError),
}

// implement From for InterpreterError
impl From<std::io::Error> for InterpreterError {
    fn from(_: std::io::Error) -> Self {
        InterpreterError::FileNotFound("the filename")
    }
}

impl From<ScanError> for InterpreterError {
    fn from(se: ScanError) -> Self {
        InterpreterError::ScanError(se)
    }
}

impl From<ParseError> for InterpreterError {
    fn from(pe: ParseError) -> Self {
        InterpreterError::ParseError(pe)
    }
}

impl From<RuntimeError> for InterpreterError {
    fn from(rte: RuntimeError) -> Self {
        InterpreterError::RuntimeError(rte)
    }
}

fn interpret_file(
    f: &PathBuf,
    show_scan: bool,
    show_parse: bool,
    eval_enabled: bool,
) -> Result<(), InterpreterError> {
    let source = fs::read_to_string(f)?;
    let tokens = scan(&source)?;
    if show_scan {
        println!("Tokens:");
        tokens.iter().for_each(|token| println!("\t{:?}", token));
    }
    let parsed = parse(&tokens)?;
    if show_parse {
        println!("\nParsed AST:\n");
        for statement in &parsed {
            println!("\t{}", statement)
        }
    }
    if eval_enabled {
        let eval_state = EvalState::new();
        eval_statements(&parsed, Rc::new(RefCell::new(eval_state)))?;
    }
    Ok(())
}

fn main() {
    let Opt {
        show_scan,
        show_parse,
        inputfile,
        eval_enabled,
    } = Opt::from_args();

    let should_eval = eval_enabled.unwrap_or(true);

    match inputfile {
        Some(f) => {
            match interpret_file(&f, show_scan, show_parse, should_eval) {
                Ok(()) => println!("Done"),
                Err(e) => println!("Error: {:?}", e),
            }
        }
        None => {
            // `()` can be used when no completer is required
            let mut rl = Editor::<()>::new().unwrap();
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
                                    if should_eval {
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
