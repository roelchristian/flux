mod lexer;

use lexer::{Lexer, Token};

fn main() {
    // get arguments from command line as file path
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        return;
    }
    let source_file = &args[1];
    let source = std::fs::read_to_string(source_file).unwrap_or_else(|_| {
        eprintln!("Error reading file: {}", source_file);
        std::process::exit(1);
    });



    let mut lexer = Lexer::new(source);

    println!("Tokens:");
    loop {
        match lexer.next_token() {
            Some(Token::EOF) => {
                println!("(end of input)");
                break;
            }
            Some(token) => println!("{:?}", token),
            None => {
                eprintln!("Lexer error: unexpected character, unable to tokenize");
                // print character
                if let Some(ch) = lexer.peek_char() {
                    println!("Unexpected character: {} on position {}", ch, lexer.get_position());
                }
                break;
            }
        }
    }
}
