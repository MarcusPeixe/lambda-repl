mod lexer;
mod parser;
mod source;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    println!("\nParser:\n");

    // Read command line arguments
    let file_name = args
        .get_mut(1)
        .expect("Error: expected source file name as first argument")
        .clone();

    // Read source code
    let source = source::Source::from_file(file_name).expect("Error: could not read source file");

    // Tokenize
    let tokens = match lexer::tokenise(&source) {
        Ok(tokens) => tokens,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    println!("Tokens:");
    for token in &tokens.tokens {
        println!("- {token:?}");
    }

    let _ast = parser::parse(&tokens);
}
