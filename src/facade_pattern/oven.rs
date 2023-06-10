pub struct Oven {}

impl Oven {
  pub fn new() -> Oven {
    Oven {  }
  }

  pub fn turn_off(&self) {
    println!("Turning off the Oven")
  }

  pub fn turn_on(&self) {
    println!("Turning on the Oven")
  }
}
