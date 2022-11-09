mod printer;

fn main() {
    printer::print_arr([ 10, 20, 30, 40]);
}

#[test]
fn should_fail() {
    unimplemented!();
}