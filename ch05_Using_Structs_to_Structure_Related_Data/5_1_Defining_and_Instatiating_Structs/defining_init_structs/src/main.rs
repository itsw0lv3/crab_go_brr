use std::fmt::Alignment;

// Tuple Structs:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

// Unit-lke Struct without any fields:
struct AlwaysEqual;

fn main() {
  let mut user1 = User {
    active: true,
    username: String::from("someUsername123"),
    email: String::from("someone@example.com"),
    sign_in_count: 1,
  };

  // Updating a value on the struct, need to be mut!
  user1.email = String::from("anewemail@example.com");

  // We can create a new instance and use a lot of the fields from user1:
  let user2 = User {
    email: String::from("anotherone@example.com"),
    ..user1
  };

  // We cannot use user1 after creation of user2 as we have moved the data!
  // We can still access the email field of user1 as it has not been moved
  // to user2

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  let subject = AlwaysEqual;
}



fn build_user(email: String, username: String) -> User {
  User {
    active: true,
    // Here we can use the shorthand
    username, // Instead of: username: username
    email, // Instead of: email: email
    sign_in_count: 1,
  }
}
