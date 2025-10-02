// Rust does not have nulls, but it does 
// have an enum that can encode the concept 
// of a value being present or absent. 
// This enum is Option<T>

// When we have a Some value, we know that 
// a value is present and the value is held 
// within the Some. When we have a None value, 
// in some sense it means the same thing as null: 
// we don’t have a valid value

fn main() {
  let some_number = Some(5);
  let some_char = Some('e');

  let absent_number: Option<i32> = None;
}
