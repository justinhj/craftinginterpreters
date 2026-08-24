use rlox::env::Environment;
use rlox::eval::RuntimeError;
use rlox::eval::eval_statements;
use rlox::parse::ParseError;
use rlox::parse::parse;
use rlox::scan::ScanError;
use rlox::scan::scan;
use rustyline::Editor;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use std::fmt;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

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
    FileNotFound(String),
    ScanError(ScanError),
    ParseError(ParseError),
    RuntimeError(RuntimeError),
    ReadlineError(ReadlineError),
}

impl From<ScanError> for InterpreterError {
    fn from(err: ScanError) -> Self {
        InterpreterError::ScanError(err)
    }
}

impl From<ParseError> for InterpreterError {
    fn from(err: ParseError) -> Self {
        InterpreterError::ParseError(err)
    }
}

impl From<RuntimeError> for InterpreterError {
    fn from(err: RuntimeError) -> Self {
        InterpreterError::RuntimeError(err)
    }
}

impl From<ReadlineError> for InterpreterError {
    fn from(err: ReadlineError) -> Self {
        InterpreterError::ReadlineError(err)
    }
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::FileNotFound(path) => write!(f, "File not found: {}", path),
            InterpreterError::ScanError(err) => write!(f, "{}", err),
            InterpreterError::ParseError(err) => write!(f, "{}", err),
            InterpreterError::RuntimeError(err) => write!(f, "{}", err),
            InterpreterError::ReadlineError(err) => write!(f, "REPL Error: {}", err),
        }
    }
}

/// Load and interpret the lox file identified by the PathBuf f
fn interpret_file(
    f: &PathBuf,
    show_scan: bool,
    show_parse: bool,
    eval_enabled: bool,
) -> Result<(), InterpreterError> {
    let source = fs::read_to_string(f)
        .map_err(|_err| InterpreterError::FileNotFound(f.clone().to_string_lossy().to_string()))?;
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
        let mut environment = Environment::new();
        eval_statements(&parsed, &mut environment)?;
    }
    Ok(())
}

fn repl(show_scan: bool, show_parse: bool, should_eval: bool) -> Result<(), InterpreterError> {
    let mut rl = Editor::<(), DefaultHistory>::new().unwrap();
    println!("Lox scanner");
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let mut environment = Environment::new();
    loop {
        let line = match rl.readline(">> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
            Err(err) => return Err(InterpreterError::ReadlineError(err)),
        };
        let _ = rl.add_history_entry(line.as_str());

        let tokens = match scan(&line) {
            Ok(tokens) => tokens,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        if show_scan {
            println!("Tokens:");
            tokens.iter().for_each(|token| println!("\t{:?}", token));
        }

        let parsed = match parse(&tokens) {
            Ok(parsed) => parsed,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        if show_parse {
            println!("\nParsed AST:\n\n");
            for statement in &parsed {
                println!("\t{}", statement)
            }
        }
        if should_eval {
            match eval_statements(&parsed, &mut environment) {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            }
        }
        let _ = rl.save_history("history.txt");
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

    let result = match inputfile {
        Some(f) => interpret_file(&f, show_scan, show_parse, should_eval),
        None => repl(show_scan, show_parse, should_eval),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
