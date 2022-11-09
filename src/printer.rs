const LENGTH: u32 = 20;
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

enum Days {
  Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

pub fn usin_enum() {
  let event_day:Days = Days::Monday;

  match event_day {
      Days::Monday => println!("The event day is Monday"),
      Days::Tuesday => println!("The event day is Tuesday"),
      Days::Wednesday => println!("The event day is Wednesday"),
      Days::Thursday => println!("The event day is Thursday"),
      Days::Friday => println!("The event day is Friday"),
      Days::Saturday => println!("The event day is Saturday"),
      Days::Sunday => println!("The event day is Sunday")
  }
}

pub fn using_constants() {
  for n in 1..LENGTH+1 {
    print!(" {}", n);
  }
}

pub fn using_tuple() { // can store value of different data type
  let tup = (29, "days", 4.3, false, ("Monday", "Tuesday", "Wednesday", "Thursday"));
  println!("{}", tup.1);
  let (mon, tues, weds, thurs) = tup.4;
  println!("Monday is {}", mon);
  println!("Tuesday is {}", tues);
  println!("Wednesday is {}", weds);
  println!("Thursday is {}", thurs);
}

pub fn print_str() {
  let mut name = "Testing.......";
  let mut first_name = String::new();
  first_name.push_str("Gbenga");
  let surname = String::from("Etomu");

  println!("Name is {}", name);
  println!("First name is {}", first_name);
  println!("Surname is {}", surname);
  println!("The name has {} characters", name.len());
}

pub fn using_array() { // can only store value of the same data time
  let arr:[i32;4] = [ 10, 20, 30, 40];

  println!("The array is {:?} & the size is {}", arr, arr.len());

  // for index in 0..arr.len() {
  //   println!("Using the range method");
  //   println!("{}", arr[index]);
  // }

  // Iter() method on array, fetches the values of all elem in an array
  println!("Using the iter method");
  for val in arr.iter() {
    println!("{}", val);
  }
}