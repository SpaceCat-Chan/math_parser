use std::env;

mod tokenize;

fn main() {
    if let Some(input) = env::args().skip(1).next() {
        let result = tokenize::run(&input);
        println!("input: {}\nresult: {:#?}", input, result);
    }
}
