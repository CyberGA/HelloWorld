/// This function is just to say hi
/**
 * prints out "Hi there"
 * This is a doc string
 */
pub fn printer_fn() {
    println!("Hi there!!");
    let example = ("dog", "cat", "horse");
    let dog = example.0;
    let cat = example.1;
    let x: f64 = 2.;

    println!("{}", dog);
    println!("{}", cat);
    println!("{}", x);
}