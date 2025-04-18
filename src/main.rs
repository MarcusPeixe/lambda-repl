mod error;
mod lexer;
mod parser;
mod source;

fn run() {

}

fn main() {
    let mut args = std::env::args();

    // Read command line arguments
    let file_name = args
        .nth(1)
        .expect("Error: expected source file name as first argument");

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

    // Parse
    let ast = match parser::parse_file(&tokens) {
        Ok(ast) => ast,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    println!("AST: {ast:#?}");

    // loop {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).expect("Failed to read line");
    //     let source = source::Source::from_string(input);
    //     let tokens = lexer::tokenise(&source);
    //     let ast = parser::parse_line(&tokens);
    //     println!("{tokens:#?}", tokens);
    // }
}
