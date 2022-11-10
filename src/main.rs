mod printer;

fn main() {
    printer::using_borrowing();
}

#[test]
fn should_fail() {
    unimplemented!();
}