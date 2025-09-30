#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}


// Defining a method on the struct
// We start with an 'impl' implication block
impl Rectangle {
  // Methods
  fn area(&self) -> u32 { // &self, shorthand for self: &Self
    self.width * self.height
  }

  fn width(&self) -> bool {
    self.width > 0
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height >  other.height
  }

  // Associated Function
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

fn main() {
  let sq = Rectangle::square(3);

  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  let rect2 = Rectangle {
    width: 10, 
    height: 40,
  };

  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };

  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

  println!(
    "The area of the rectangle is : {} sqaure pixels.",
    rect1.area()
  );

  if rect1.width() {
    println!("The rectanlge has a nonzero width; it is {}", rect1.width);
  }

  println!("The area of 'sq': is {}", sq.area());

}
