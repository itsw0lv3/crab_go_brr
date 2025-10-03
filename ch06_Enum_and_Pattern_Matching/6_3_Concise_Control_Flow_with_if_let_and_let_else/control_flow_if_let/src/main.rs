use std::os::linux::raw::stat;

#[derive(Debug)]
enum UseState {
  Alabama,
  Alaska,
  // more
}

impl UseState {
  fn existed_in(&self, year: u16) -> bool {
    match self {
      UseState::Alaska => year >= 1959,
      UseState::Alabama => year >= 1819,
    }
  }
}

fn describe_state_quater(coin: Coin) -> Option<String> {
  let state = if let Coin::Quater(state) = coin {
    state
  } else {
    return None;
  };

  if state.existed_in(1900) {
    Some(format!("{state:?} is pretty old, for America!"))
  } else {
      Some(format!("{state:?} is relatively new."))
  }
}


enum Coin {
  Penny,
  Nickel,
  Dime,
  Quater(UseState),
}

fn main() {
  let config_max = Some(3u8);
  
  // Lets change this to if let:
  // match config_max {
  //   Some(max) => println!("The maximum is configured to be: {max}"),
  //   _ => (),
  // }

  // if let
  if let Some(max) = config_max {
    println!("The maximum is configured to be: {max}");
  }

  // More examples, lets move this to let if else:
  // let mut count = 0;
  // match coin {
  //  Coin::Quater(state) => println!("State quater from {"state:?}!"),
  //  _ => count += 1,
  // }
  
  // if let and else:
  let coin = Coin::Quater(UseState::Alaska);
  let mut count = 0;

  if let Coin::Quater(state) = coin {
    println!("State quarter from {state:?}!");
  } else {
    count += 1;
  }
}
