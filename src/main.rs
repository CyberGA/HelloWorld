mod printer;

fn main() {
    let mut arr = [ 10, 20, 30, 40];
    printer::print_arr(&mut arr);
}

#[test]
fn should_fail() {
    unimplemented!();
}