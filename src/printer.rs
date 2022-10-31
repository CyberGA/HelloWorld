/// This function is just to say hi
/**
 * prints out "Hi there"
 * This is a doc string
 */
pub fn printer_fn() {
    println!("Hi there!!");
    let example = ("dog", "cat", "horse");
    let dog:&str = example.0;
    let cat = example.1;
    let x:u64 = 0;
    let f:f32 = 3.2;
    let answer:bool = false;

    if x > 0 && answer == false && f == 3.2 {
        println!("{}", dog);
        println!("{}", cat);
        println!("{}", x);
    } else if f == 3.2 {
        println!("Floating")
    } else {
        println!("Booling")
    }

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

pub fn looping() {
  let mut x = 0;

  loop {
      if x == 10 {
        break;
      }

      println!("Value is - {}", x);
      x += 1;
  }
}

pub fn loop_while() {
  let mut n = 0;

  while n < 11 {
      println!("{}", n);
      n += 1;
  }
}

pub fn loop_for() {
  let cars = vec!["Benz", "Audi", "Ferrari"];
  for (index, car) in cars.iter().enumerate() {
    println!("{} is at index {}", car, index)
  }
}