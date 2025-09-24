fn main() {
  loop_labes();
}

fn if_express() {
  let number = 3;

  if number < 5 {
    println!("condition was true!");
  } else {
    println!("condition was false!");
  }
}


// This will throw an error as the if statement expects a bool value
// Rust will no automatically convert non-Booleans to a bool
// This must be explicit
fn wrong_if() {
  let number = 3;

  // Wrong! expect bool, got int
  // if number {
  //   println!("The number is three");
  // }

  if number != 0 {
    println!("Number was something other than zero!");
  }
}


// Multiple Conditions
fn multi_cond() {
  let num = 6;

  if num % 4 == 0 {
    println!("number is divisible by 4");
  } else if num % 3 == 0 {
    println!("number is divisible by 3");
  } else if num % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }
}


// If in let statement:
fn if_let() {
  let condition = true;

  let num = if condition { 5 } else { 6 };

  println!("The value of the number is: {num}");
}


// Repeatition with loops:
fn with_loop() {
  loop {
    println!("again");
  }
}


// returning values form loops:
fn return_loop() {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      break counter * 2;
    }
  };

  println!("The result is {result}");
}


// Loop Lables:
fn loop_labes() {
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }

      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }
    count += 1;
  }
  println!("End count: {count}");
}


// Conditional Loops with while
fn cond_while() {
  let mut num = 3;

  while num != 0 {
    println!("{num}");

    num -= 1;
  }

  println!("LIFTOFF!!!");
}


// Looping through a collection with for:
fn for_loop_thro() {
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("the value is : {}", a[index]);

    index += 1;
  }
}

// Alternative to looping a collection:
fn for_loop_alt_one() {
  let a = [10, 20, 30, 40, 50];

  for element in a {
    println!("the value is: {element}");
  }
}


fn for_loop_alt_two() {
  for num in (1..4).rev() {
    println!("{num}");
  }
  println!("LIFTOFF!");
}



