// Lox code scanner using nom
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::fs;
use rlox::scan;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if !args.is_empty() {
        let source = fs::read_to_string(&args[0]).unwrap();

        match scan(&source) {
            Ok(tokens) => tokens.iter().for_each(|token| println!("{:?}", token)),
            Err(err) => println!("Error {:?}", err),
        }
    } else {
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
                        rl.add_history_entry(line.as_str());
                        tokens.iter().for_each(|token| println!("{:?}", token))
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
