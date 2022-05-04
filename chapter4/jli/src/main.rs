// The library code is really just a scanner and contains not code load files and so on. This
// executable handles the loading of scripts or scanning from input.
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 0 {
        let source = fs::read_to_string(&args[0]).unwrap();

        match jli::scan(&source) {
            Ok(tokens) => tokens.iter().for_each(|token| println!("{:?}", token)),
            Err(err) => println!("Error {:?}", err),
        }
    } else {
        let input = "\
            fun addPair(a, b) {\n\
              c = 3.14;\n\
              return a + b * c;\n\
            }";

        match jli::scan(input) {
            Ok(tokens) => tokens.iter().for_each(|token| println!("{:?}", token)),
            Err(err) => println!("Error {:?}", err),
        }
        println!("Pass a filename")
    }
}
