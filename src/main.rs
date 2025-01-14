mod lexer;
mod parser;
mod source;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    println!("\nParser:\n");

    // Read command line arguments
    let file_name = args
        .get_mut(1)
        .expect("Error: expected source file name as first argument");

    let file_name = std::mem::take(file_name);

    // Read source code
    let code = std::fs::read_to_string(&file_name).expect("Error: could not read source file");

    // Construct source object
    let source = source::Source::new(file_name, code);

    // Tokenize
    let tokens = lexer::tokenise(&source);

    match tokens {
        Ok(tokens) => {
            println!("Tokens:");
            for token in tokens.tokens {
                println!("- {token:?}");
            }
        }
        Err(error) => {
            println!("{error}");
        }
    };
}
