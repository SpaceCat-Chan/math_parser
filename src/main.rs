use std::env;

mod parse_tree;
mod tokenize;

fn main() {
    if let Some(input) = env::args().skip(1).next() {
        let result = tokenize::run(&input);
        println!("input: {}\nresult: {:#?}", input, result);
        let result = match result {
            Ok(x) => x,
            Err(x) => {
                print_error_line(input, x.1, &x.0);
                return;
            }
        };
        let tree = parse_tree::run(&result);
        let tree = match tree {
            Ok(x) => x,
            Err(x) => {
                print_error_line(input, &x.0, x.1);
                return;
            }
        };
        println!("parsed as: {:#?}\nresulting in {}", tree, tree.eval());
    }
}

fn print_error_line(input: String, message: &str, whence: &tokenize::Token) {
    let result = input.lines().skip(whence.line).next();
    let result = match result {
        Some(x) => x.to_string(),
        None => "?".to_string().repeat(whence.line_pos.1),
    };
    let line_number = whence.line.to_string();
    println!("     | Error: {}:", message);
    println!(
        "{}{} | {}",
        " ".repeat(4 - line_number.len()),
        line_number,
        result
    );
    println!(
        "     | {}{}",
        " ".repeat(whence.line_pos.0),
        "~".repeat(whence.line_pos.1 - whence.line_pos.0)
    );
}
