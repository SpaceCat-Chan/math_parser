mod tokenize;

fn main() {
    println!("Hello, world!");
    let a = Token {
        token_type: TokenType::NUMBER(64.0),
        raw: String::from("64"),
        line: 0,
        line_pos: (0, 2),
    };
}
