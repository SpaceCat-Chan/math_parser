use super::tokenize;

#[derive(Debug)]
pub enum NodeAction {
    Number(f64),
    Add(BinaryNode),
    Subtract(BinaryNode),
    Multiply(BinaryNode),
    Divide(BinaryNode),
    Negate(UnaryNode),
    Identifier(String),
}

#[derive(Debug)]
pub struct BinaryNode {
    left: Box<NodeAction>,
    right: Box<NodeAction>,
}

#[derive(Debug)]
pub struct UnaryNode {
    node: Box<NodeAction>,
}

impl NodeAction {
    pub fn eval(&self) -> f64 {
        use NodeAction::*;
        match self {
            Identifier(_) => todo!("Parsing identifiers"),
            Add(node) => node.left.eval() + node.right.eval(),
            Subtract(node) => node.left.eval() - node.right.eval(),
            Multiply(node) => node.left.eval() * node.right.eval(),
            Divide(node) => node.left.eval() / node.right.eval(),
            Negate(node) => -node.node.eval(),
            Number(number) => *number,
        }
    }
}

type Error<'a> = (String, &'a tokenize::Token);

pub fn run(tokens: &Vec<tokenize::Token>) -> Result<NodeAction, Error> {
    let mut parse_point = 0;
    parse_expression(&tokens[..], &mut parse_point)
}

fn parse_expression<'a>(
    tokens: &'a [tokenize::Token],
    parse_point: &mut usize,
) -> Result<NodeAction, Error<'a>> {
    parse_term(tokens, parse_point)
}

fn parse_term<'a>(
    tokens: &'a [tokenize::Token],
    parse_point: &mut usize,
) -> Result<NodeAction, Error<'a>> {
    let mut current = parse_factor(tokens, parse_point)?;
    loop {
        if *parse_point >= tokens.len() {
            return Ok(current);
        }
        match tokens[*parse_point].token_type {
            tokenize::TokenType::Minus => {
                *parse_point += 1;
                let next = parse_factor(tokens, parse_point)?;
                current = NodeAction::Subtract(BinaryNode {
                    left: Box::new(current),
                    right: Box::new(next),
                });
            }
            tokenize::TokenType::Plus => {
                *parse_point += 1;
                let next = parse_factor(tokens, parse_point)?;
                current = NodeAction::Add(BinaryNode {
                    left: Box::new(current),
                    right: Box::new(next),
                });
            }
            _ => return Ok(current),
        }
    }
}

fn parse_factor<'a>(
    tokens: &'a [tokenize::Token],
    parse_point: &mut usize,
) -> Result<NodeAction, Error<'a>> {
    let mut current = parse_unary(tokens, parse_point)?;
    loop {
        if *parse_point >= tokens.len() {
            return Ok(current);
        }
        match tokens[*parse_point].token_type {
            tokenize::TokenType::Star => {
                *parse_point += 1;
                let next = parse_unary(tokens, parse_point)?;
                current = NodeAction::Multiply(BinaryNode {
                    left: Box::new(current),
                    right: Box::new(next),
                });
            }
            tokenize::TokenType::Divide => {
                *parse_point += 1;
                let next = parse_unary(tokens, parse_point)?;
                current = NodeAction::Divide(BinaryNode {
                    left: Box::new(current),
                    right: Box::new(next),
                });
            }
            _ => return Ok(current),
        }
    }
}

fn parse_unary<'a>(
    tokens: &'a [tokenize::Token],
    parse_point: &mut usize,
) -> Result<NodeAction, Error<'a>> {
    if *parse_point >= tokens.len() {
        return Err((
            "Expected unary expression after".into(),
            &tokens[*parse_point - 1],
        ));
    }
    match tokens[*parse_point].token_type {
        tokenize::TokenType::Minus => {
            *parse_point += 1;
            let parsed = parse_unary(tokens, parse_point)?;
            return Ok(NodeAction::Negate(UnaryNode {
                node: Box::new(parsed),
            }));
        }
        _ => parse_primary(tokens, parse_point),
    }
}

fn parse_primary<'a>(
    tokens: &'a [tokenize::Token],
    parse_point: &mut usize,
) -> Result<NodeAction, Error<'a>> {
    if *parse_point >= tokens.len() {
        return Err((
            "Expected primary expession after".into(),
            &tokens[*parse_point - 1],
        ));
    }
    match &tokens[*parse_point].token_type {
        tokenize::TokenType::Number(x) => {
            *parse_point += 1;
            Ok(NodeAction::Number(*x))
        }
        tokenize::TokenType::Identifier(x) => {
            *parse_point += 1;
            Ok(NodeAction::Identifier(x.clone()))
        }
        tokenize::TokenType::Parenthisies(tokenize::Side::Left) => {
            *parse_point += 1;
            let result = parse_expression(tokens, parse_point)?;
            if *parse_point >= tokens.len() {
                return Err((
                    "Expected right parentheses, but got EOF".into(),
                    &tokens[*parse_point - 1],
                ));
            }
            let next = &tokens[*parse_point];
            if next.token_type != tokenize::TokenType::Parenthisies(tokenize::Side::Right) {
                return Err(("Expected right parentheses".into(), next));
            }
            *parse_point += 1;
            return Ok(result);
        }
        tokenize::TokenType::Parenthisies(tokenize::Side::Right) => {
            Err(("Unexpected right parentheses".into(), &tokens[*parse_point]))
        }
        _ => Err(("Expected Primary Expression".into(), &tokens[*parse_point])),
    }
}
