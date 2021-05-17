#[derive(PartialEq, Debug)]
enum Side {
    Right,
    Left,
}

#[derive(PartialEq, Debug)]
enum TokenType {
    Number(f64),
    Plus,
    Minus,
    Star,
    Divide,
    Identifier(String),
    Parenthisies(Side),
}

impl TokenType {
    fn from(string: &str) -> TokenType {
        TokenType::Identifier(String::from(string))
    }
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    line: usize,
    line_pos: (usize, usize),
}

impl Token {
    fn new(token_type: TokenType, token_pos: TokenPos) -> Token {
        Token {
            token_type: token_type,
            line: token_pos.line,
            line_pos: (token_pos.start, token_pos.end),
        }
    }
}

fn single_token_char(character: char) -> bool {
    match character {
        '+' | '-' | '*' | '/' | '(' | ')' => true,
        _ => false,
    }
}

#[derive(Clone)]
struct TokenPos<'a> {
    content: &'a str,
    line: usize,
    start: usize,
    end: usize,
}

fn split_whitespace_to_tokens(string: &String) -> Vec<TokenPos> {
    let mut string_tokens: Vec<TokenPos> = vec![];
    for (line_index, line) in string.lines().enumerate() {
        let mut cur_start: Option<usize> = None;
        for (char_index, character) in line.chars().enumerate() {
            if character.is_whitespace() {
                if let Some(start) = cur_start {
                    string_tokens.push(TokenPos {
                        content: &line[start..char_index],
                        line: line_index,
                        start,
                        end: char_index,
                    });
                    cur_start = None;
                }
                continue;
            } else {
                if cur_start.is_none() {
                    cur_start = Some(char_index);
                }
            }
        }
        if let Some(start) = cur_start {
            let char_count = line.chars().count();
            string_tokens.push(TokenPos {
                content: &line[start..char_count],
                line: line_index,
                start,
                end: char_count,
            });
        }
    }
    string_tokens
}

fn split_inside_tokens<'a>(tokens: &Vec<TokenPos<'a>>) -> Vec<TokenPos<'a>> {
    let mut result: Vec<TokenPos<'a>> = vec![];

    for token in tokens {
        let mut split_tokens: Vec<TokenPos> = vec![];
        if token.content.chars().count() <= 1 {
            split_tokens.push(token.clone());
        } else {
            let mut current_token = token.clone();
            let mut current_char_start = 0;
            let mut previous_was_single = false;
            current_token.end = current_token.start;
            for (char_index, character) in token.content.chars().enumerate() {
                if single_token_char(character) {
                    if current_token.start != current_token.end {
                        current_token.content = &token.content[current_char_start..char_index];
                        split_tokens.push(current_token);
                    }
                    let token_start = token.start + char_index;
                    current_token = TokenPos {
                        content: &token.content[char_index..char_index + 1],
                        line: token.line,
                        start: token_start,
                        end: token_start + 1,
                    };
                    current_char_start = char_index;
                    previous_was_single = true;
                    continue;
                }
                if previous_was_single {
                    previous_was_single = false;
                    current_token.content = &token.content[current_char_start..char_index];
                    split_tokens.push(current_token);
                    let token_start = token.start + char_index;
                    current_token = TokenPos {
                        content: &token.content[char_index..char_index + 1],
                        line: token.line,
                        start: token_start,
                        end: token_start + 1,
                    };
                    current_char_start = char_index;
                    continue;
                }
                current_token.end += 1;
            }
            split_tokens.push(current_token);
        }
        result.append(&mut split_tokens);
    }

    result
}

fn give_tokens_types(token_pos: Vec<TokenPos>) -> Result<Vec<Token>, (Token, &'static str)> {
    let mut tokens = vec![];
    for token in token_pos {
        let parse_attempt = token.content.parse::<f64>();
        if let Ok(number) = parse_attempt {
            tokens.push(Token::new(TokenType::Number(number), token));
            continue;
        }
        if token.content.chars().all(|c| c.is_alphanumeric()) {
            tokens.push(Token::new(TokenType::from(token.content), token));
            continue;
        }
        if token.content.chars().count() != 1 {
            return Err((
                Token::new(TokenType::from(token.content), token),
                "Invalid token",
            ));
        }
        tokens.push(match token.content.chars().next().unwrap() {
            '+' => Token::new(TokenType::Plus, token),
            '-' => Token::new(TokenType::Minus, token),
            '*' => Token::new(TokenType::Star, token),
            '/' => Token::new(TokenType::Divide, token),
            '(' => Token::new(TokenType::Parenthisies(Side::Left), token),
            ')' => Token::new(TokenType::Parenthisies(Side::Right), token),
            _ => {
                return Err((
                    Token::new(TokenType::from(token.content), token),
                    "Invalid token",
                ))
            }
        });
    }

    Ok(tokens)
}

pub fn run(string: &String) -> Result<Vec<Token>, (Token, &'static str)> {
    let token_pos = split_whitespace_to_tokens(string);

    let token_pos = split_inside_tokens(&token_pos);

    give_tokens_types(token_pos)
}
