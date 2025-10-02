fn main() {
  let dice_roll = 9;
  match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // Match catch all, call empty tuple
  }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
