mod printer;

fn main() {
    printer::proprinter(10);
    printer::proprinter("Gbenga Etomu");
    printer::proprinter('A');
    printer::proprinter(true);
    printer::proprinter(10.2);
}

#[test]
fn should_fail() {
    unimplemented!();
}