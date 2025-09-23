fn main() {
  // Unsigned and Signed Integers:
  let a : u16 = 300;
  println!("{a}");
  // unsigned Int for only positives, signed for use of negatives, check the documentation for size
  // needed.
  // We need a "sign = -" to show a negative number.
  
  // Floating Point Numbers:
  let b : f64 = 2.0; // f64
  let c : f32 = 3.0; // f32
  // By default most floats will be f64 due to most systems being 64 bit.
  println!("f64 is {b}, f32 is {c}");
  

  // Numeric Operations:
  let sum = 5 + 10;
  let difference = 95.5 - 4.3;
  let product = 4 * 30;
  let quotient = 56.7 / 32.2;
  let remainder = 43 % 5;

  println!("The sum of 5 + 10 is: {sum}");
  println!("The difference of 95.5 - 4.3 is: {difference}");
  println!("The product of 4 * 30 is: {product}");
  println!("The quotient of 56.7 / 32.2 is: {quotient}");
  println!("The remainder of 43 % 5 is: {remainder}");

  // Booleans
  let t = true;
  let f: bool = false; // with explicit type annotation

  println!("Bools are {t} or {f}");

  // Character Type:
  // char is put in ' ' as opposed to string literals which are encased in " "
  // chars are four bytes in size and represents a Unicode scalar value, which means it can
  // represent more than just ASCII.
  // Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are
  // all valid.
  
  let c = 'z';
  let z: char = 'Z';
  let heart_eyed_cat = '😻';

  println!("here are some chars: {c}, {z}, and {heart_eyed_cat}");

  // Compound Types:
  // A. The Tuple Types:
  
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  println!("{:?}", tup);

  // Deconstructing the tuple to multiple vars:
  let (one, two, three) = tup;
  println!("one is: {one}");
  println!("two is: {two}");
  println!("three is: {three}");

  // We can also access an element with the . followed by index:
  let tup_idx_one = tup.0;
  println!("tup_idx_one is the first element in the tuple with the value of: {tup_idx_one}");

  // B. Array Type:
  let arr = [1, 2, 3, 4, 5];
  let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
  let arr_two: [i32; 5] = [1, 2, 3, 4, 5]; // with type
  // You can also init an array to contain the same value for each element by specifying the
  // initial value, followed by a semicolon and then the length on the array
  let arr_three = [3; 5]; // [3, 3, 3, 3, 3]

  // Accessing index and assigning it to a variable:
  let first_idx = arr[0];
}
