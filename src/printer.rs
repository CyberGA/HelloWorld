use std::{collections::{HashMap, HashSet}, any, fs::File, fmt::Display, io::Write};

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
  let mut arr:[i32;4] = [ 10, 20, 30, 40];

  // println!("The array is {:?} & the size is {}", arr, arr.len());

  // for index in 0..arr.len() {
  //   println!("Using the range method");
  //   println!("{}", arr[index]);
  // }

  // Iter() method on array, fetches the values of all elem in an array
  // println!("Using the iter method");
  // for val in arr.iter() {
  //   println!("{}", val);
  // }

  // Making array mutable
    arr[3] = 80;
    // println!("The array is {:?} after changes", arr);

    // using constants with array
    const N: usize = 20; 
   // pointer sized
    let nums_arr = [0; N];

    println!("The array is {:?} & the size is {}", nums_arr, nums_arr.len())
}

/**
 * passing array as parameters to function
 */
// pub fn print_arr(mut arr:[i32;4]) { // pass by value
pub fn print_arr(arr: &mut[i32;4]) {
  println!("The array is : {:?}", arr);
}

/**
 * Borrowing example
 */

 pub fn using_borrowing() {

  // this will not result in error
  // because the ownership has been transferred
//   let v = vec![10,20,30];
//   print_vector(v);
//   println!("{}", v[0]); // this line gives error

//   fn print_vector(x:Vec<i32>){
//     println!("Inside print_vector function {:?}",x);
//  }

  // this will working because we passed by reference
  // and the ownership is transferred back to the originally object
  // let v = vec![10,20,30];
  // print_vector(&v); // passing reference
  // println!("Printing the value from main() v[0]={}",v[0]);

  // fn print_vector(x:&Vec<i32>){
  //   println!("Inside print_vector function {:?}",x);
  // }

  // function can modify a borrowed resource by using a mutable reference to such resource
  // fn add_one(e: &mut i32) {
  //   *e+= 1;
  // }
  // let mut i = 3;
  //  add_one(&mut i);
  //  println!("{}", i);


  // mutating a string
  fn display(param_name:&mut String){
    println!("param_name value is :{}",param_name);
    param_name.push_str(" Rocks"); 
    //Modify the actual string,name
   }
   let mut name:String = String::from("TutorialsPoint");
   display(&mut name); 
   //pass a mutable reference of name
   println!("The value of name after modification is:{}",name);
 }

 /**
  * Slicing
  */
pub fn using_slice() {
  // let n1 = "Tutorials".to_string();
  // println!("The length of the string is {}", n1.len());
  // let c1 = &n1[4..9];
  // println!("{}", c1);

  let mut data = [10,20,30,40,50];
  use_slice(&mut data[1..4]);
  println!("The length of the array is {:?}", data.len());

  fn use_slice(slice:&mut [i32]) { 
    // is taking a slice or borrowing a part of an array of i32s
    println!("length of slice is {:?}", slice.len());
    slice[0] = 1010;
    println!("{:?}",slice); 
  }
}


pub fn using_struct() {
  struct Person {
    first_name:String,
    last_name:String,
    tel:String,
    years:i32
  }

  let mut father = Person {
    first_name: String::from("Vincent"),
    last_name: String::from("Etomu"),
    tel: String::from("+2349031741560"),
    years: 54
  };

  father.years = 56;

  println!("Father's name is {} {} and {} years old, his telephone is {}", father.first_name, father.last_name, father.years, father.tel);

  println!(">>>>>>>>>>>>>>>>>>>>>>>>>>\n");

  fn display(person:Person) {
    println!("Father's name is {} {} and {} years old, his telephone is {}", person.first_name, person.last_name, person.years, person.tel);
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>");
  }

  display(father);
}

pub fn method_in_struct() {
  struct Rectangle {
    width: u32, height: u32
  }
  impl  Rectangle {
      fn area(&self) {
        println!("The area is {}", self.width * self.height)
      }
  }
  let wall = Rectangle{width: 4, height: 8};
  wall.area();
}

pub fn static_struct_method() {
  struct Rectangle {
    width: u32, height: u32
  }
  impl  Rectangle {
      fn areaInstance(w: u32, h: u32) -> Rectangle {
        Rectangle { width: w, height: h}
      }
      fn area(&self) {
        println!("The area is {}", self.width * self.height)
      }
  }
  let wall = Rectangle::areaInstance(4, 8);
  wall.area();
}

pub fn option_enum() {
  fn is_even(no: i32) -> Option<bool> {
    if no % 2 == 0 {
      Some(true)
    } else {
        None
    }
  }

  match is_even(5) {
      Some(data) => {
        if data == true {
          println!("Even number");
        }
      },
      None => {
        println!("Not even number")
      }
  }
}

pub fn using_vector() {
  // let mut v = Vec::new();
  let mut v = vec![10, 20, 30, 40];
  v.remove(0);
  v.push(15);
  v.push(25);
  v.push(35);

  println!("The size of the vector is {}", v.len());
  println!("{:?}\n", v);

  for (index, val) in v.iter().enumerate() {
    println!("{} is at {}", val, index)
  }
}

pub fn using_hashmap() {
  // let mut state_codes:HashMap<&str, &str> = HashMap::new();
  let mut state_codes = HashSet::new();
  state_codes.insert("US");
  state_codes.insert("NG");
  state_codes.insert("GH");
  state_codes.insert("SA");

  println!("The size of the set is {}\n", state_codes.len());

  println!("{:?}\n", state_codes);

  match state_codes.get(&"US") {
      Some(val) => {
        println!("Value US is {}", val);
      }
      None => {
        println!("Nothing is found");
      }
  }

  state_codes.remove(&"GH");

  for val in state_codes.iter() {
    println!("value is {}\n", val )
  }

  if state_codes.contains(&"NG") {
    println!("Found key")
  }
}

pub fn working_on_errors() -> Result<bool, String> {
  if 5%2==0 {
      return Ok(true);
   } else {
      return Err("NOT_AN_EVEN".to_string());
   }
}

pub fn using_generics() {
  let mut ages: Vec<u32> = vec![12, 13, 18, 20];
  ages.push(40);
  println!("{:?}\n\n", ages);

  struct  Data<T> {
    value: T
  }

  let t:Data<&str> = Data { value: "Gbenga" };
  let person:Data<String> = Data { value: "Gbenga Joshua Etomu".to_string() };
  println!("The data value is: {}", t.value);
  println!("The person's name is: {}", person.value);
}

pub fn using_traits() {
  struct Book {
    name: &'static str,
    id: u32
  }

  trait Printable {
      fn disp(&self);
  }

  impl Printable for Book {
    fn disp(&self) {
      println!("Book name: {} and Book Id: {}", self.name, self.id)
    }
  }

  let book: Book = Book { name: "The Blacklist", id: 1229 };
  book.disp();
}

// generic functions
pub fn proprinter<T:Display>(t:T) {
  println!("Inside the proprinter generic function");
  println!("{}", t);
}

pub fn read_write() {
  let mut line = String::new();
  println!("Enter your fullname:");

  let b1 = std::io::stdin().read_line(&mut line).unwrap();
  println!("Hello, {}", line);
  println!("No of bytes read: {}\n\n", b1);

  let output1 = std::io::stdout().write("Hello, World!\n".as_bytes()).unwrap();
  let output2 = std::io::stdout().write("Welcome to rust programming\n".as_bytes()).unwrap();
  std::io::stdout().write(format!("\nbytes written {}\n\n", (output1+output2)).as_bytes()).unwrap();

  // get command line arg
  let cmd_line = std::env::args();
  println!("No of elements in args is: {}", cmd_line.len());
  
  for arg in cmd_line {
    println!("[{}]", arg);
  }
}

pub fn working_with_file() {
  let mut file = std::fs::File::create("./src/data.txt").expect("creating file failed");
  file.write_all("Hello, World!".as_bytes()).expect("Writing failed");
  file.write_all("\nWelcome to Rust Programming!!".as_bytes()).expect("writing failed");
  println!("Data successfully written");
}