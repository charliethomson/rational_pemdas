mod lex;
mod tree;
mod value;
use std::io::Write;
use value::*;

fn calc(s: &String) -> Value {
    let tree = tree::Tree::new(&s);
    tree.evaluate()
}

fn main() {
    println!("{:#?}", Value::from(13.5));

    let input = std::io::stdin();
    let mut output = std::io::stdout();
    let mut buffer = String::new();
    println!("Enter an expression");
    loop {
        print!(">> ");
        output.flush().unwrap();
        input.read_line(&mut buffer).unwrap();
        println!("Result: {}", calc(&buffer));
        buffer.clear();
    }
}
