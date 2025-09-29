fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Using the reference symbol: &
    println!("The value of '{s1}' is {len}");

    let mut s = String::from("hello");
    change(&mut s);

    // We cannot borrow twice for mutable references, 
    // you can only have a single borrow for a single reference
    
    // This will error:
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}, {r2}");
  
    // We can borrow multiple instances of immutable:
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

    dangle();
}


// Using a reference instead of transfering ownership
// A reference is like a pointer in that it's an address 
// we can follow to access the stored data at that address.
//
// Unlike a pointer, a reference is guaranteed to point
// to a valid value of a particular type for the life
// of that reference
fn calculate_length(s: &String) -> usize { // This uses a reference with the 
                                           // '&' meaning we do not have
                                           // to return the value, as we 
                                           // do not take ownership.
  s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}

// fn dangle() -> &String {
fn dangle() -> String {  
  let s  = String::from("hello");

  // This will result in an error, covered later, lifetimes
  // &s

  s
}
