fn main() {
  println!("Hello, world!");

  another_function(5);
  print_label_measurement(5, 'h');

  // Use expressions in the block scope
  let y = {
    let x = 3;
    x + 1
  };
  println!("The value of y is: {y}");


  let val = five();
  println!("value of five fn is: {val}");

  let add_one_res = add_one(5);
  println!("The result of add_one(5) is: {add_one_res}");
}

// A function with an argument:
fn another_function(x: i32) {
  println!("The value of x is: {x}");
}

// Multiple arguements:
fn print_label_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value} - {unit_label}");
}

// Functions with a return value:
fn five() -> i32 {
  // We do not need a semi colon becuase it is an expression
  // we want to return
  5
}

// Here is another return with an argument passed to it:
fn add_one(x: i32) -> i32 {
  x + 1
}
