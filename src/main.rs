mod printer;

fn main() {
    let res = printer::working_on_errors().unwrap();
    println!("The result is : {}", res)
    
}

#[test]
fn should_fail() {
    unimplemented!();
}