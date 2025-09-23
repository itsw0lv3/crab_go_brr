use std::io;

fn main() {
  // If the index is greater than or equal to the length, Rust will panic. 
  // This check has to happen at runtime, especially in this case, because the compiler
  // can’t possibly know what value a user will enter when they run the code later.
  // This is an example of Rust’s memory safety principles in action.
  // In many low-level languages, this kind of check is not done, and when you provide an incorrect index, 
  // invalid memory can be accessed. 
  // Rust protects you against this kind of error by immediately exiting 
  // instead of allowing the memory access and if continuing.
  
  let a = [1, 2, 3, 4, 5];
  println!("Please enter an array index");

  let mut index = String::new();

  io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line.");

  let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

  let element = a[index];

  println!("The value of the element at index {index} is: {element}");

  // This will cause an issue if the index the user provides is larger than the array.
}
