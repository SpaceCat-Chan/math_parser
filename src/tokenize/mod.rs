pub mod tokenize {
    enum Side {
        RIGHT,
        LEFT,
    }

    enum TokenType {
        NUMBER(f64),
        PLUS,
        MINUS,
        STAR,
        DIVIDE,
        IDENTIFIER(String),
        PARENTHESIES(Side),
    }

    struct Token {
        token_type: TokenType,
        raw: String,
        line: usize,
        line_pos: (usize, usize),
    }

    fn submit_token(token: Option<Token>, tokens: &mut Vec<Token>, end: usize) {
        if let Some(current) = token.take() {
            current.line_pos.1 = end;
            tokens.push(current);
        }
    }

    pub fn run(string: String) -> Vec<Token> {
        let tokens = vec![];
        for (line_index, line) in string.lines().enumerate() {
            let current: Option<Token>;
            for (char_index, curr) in line.chars().enumerate() {
                if curr.is_whitespace() {
                    submit_token(current, &mut tokens, char_index);
                }
            }
            submit_token(current, &mut tokens, line.len());
        }
        tokens
    }
}
