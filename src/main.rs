mod printer;

fn main() {
    println!("Hello, world!");
    println!("Welcome to RUST programming!!");
    printer::printer_fn();
    printer::machine_type();
}

#[test]
fn should_fail() {
    unimplemented!();
}