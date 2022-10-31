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

pub fn machine_type() {
    // Check the type of maching
    let machine_kind = if cfg!(unix) {
        "unix"
      } else if cfg!(windows) {
        "windows"
      } else {
        "unknown"
      };
    println!("Your machine type is {}", machine_kind);
}