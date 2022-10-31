mod printer;

fn main() {
    println!("Hello, world!");
    println!("Welcome to RUST programming!!");
    printer::printer_fn()
}

#[test]
fn should_fail() {
    unimplemented!();
}