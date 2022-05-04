// The library code is really just a scanner and contains not code load files and so on. This
// executable handles the loading of scripts or scanning from input.

fn main() {
    let input = "\
            fun addPair(a, b) {\n\
              c = 3.14;\n\
              return a + b * c;\n\
            }";

    match jli::scan(input) {
        Ok(tokens) => tokens.iter().for_each(|token| println!("{:?}", token)),
        Err(err) => println!("Error {:?}", err),
    }
}
