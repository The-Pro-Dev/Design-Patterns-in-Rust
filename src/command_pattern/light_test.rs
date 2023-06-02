#[cfg(test)]
mod light_test {
  use crate::command_pattern::light::Light;

  #[test]
  fn turn_on() {
    let mut light = Light::new();
    light.turn_on();
    assert_eq!(light.is_on(), true);
  }

  #[test]
  fn turn_off() {
    let mut light = Light::new();
    light.turn_off();
    assert_eq!(light.is_on(), false);
  }
}
