mod printer;

fn main() {
    printer::proprinter(10 as u32);
    printer::proprinter(20 as u16);
    printer::proprinter(30 as u8);
    printer::proprinter("Test");
}

#[test]
fn should_fail() {
    unimplemented!();
}