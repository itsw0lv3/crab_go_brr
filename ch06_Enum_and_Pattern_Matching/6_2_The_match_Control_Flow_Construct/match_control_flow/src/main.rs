#![allow(dead_code)]

#[derive(Debug)]
enum UseState {
  Alabama,
  Alaska,
  // more
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quater(UseState),
}

fn main() {
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin:: Penny => {
      println!("Lucky Penny!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quater(state) => {
      println!("State quarter from {state:?}!");
      25
    },
  }
}
