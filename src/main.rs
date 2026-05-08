mod lexer;

use lexer::lexer::Lexer;
use lexer::token::TokenType;

fn main() {
    let source = "const x: number = 5;".to_string();

    let mut lexer = Lexer::new(source);

    // loop {
        let token = lexer.next_token();

        println!("{:?}", token);

        // if token.token_type == TokenType::EOF {
        //     break;
        // }
    // }
}