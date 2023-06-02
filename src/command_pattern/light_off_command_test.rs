#[cfg(test)]
mod light_off_command_test {
  use crate::command_pattern::{light::Light, light_off_command::LightOffCommand, command::Command};

  #[test]
  fn execute() {
    let mut light = Light::new();
    let mut light_off_command = LightOffCommand::new(&mut light);
    light_off_command.execute();
    assert_eq!(light.is_on(), false);
  }
}
