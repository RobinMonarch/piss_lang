mod lexer_parser;

fn main() {
    println!("Hello, world!\n\n");
    let sus = lexer_parser::lex_file("C:/Users/Joshua/Downloads/~ Demonstration of PISS.txt");
    println!("{:?}", sus);
    lexer_parser::parse_sentence(&sus[0]);
}
