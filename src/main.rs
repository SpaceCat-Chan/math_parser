mod tokenize;

fn main() {
    let input = String::from("(4 + 2+4)/ 3");
    let result = tokenize::run(&input);
    println!("input: {}\nresult: {:#?}", input, result);
}
