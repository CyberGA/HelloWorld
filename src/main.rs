mod printer;

fn main() {
    let res = printer::working_on_errors().unwrap();
    
}

#[test]
fn should_fail() {
    unimplemented!();
}