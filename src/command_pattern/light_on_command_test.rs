#[cfg(test)]
mod light_on_command_test {
  use crate::command_pattern::{light::Light, light_on_command::LightOnCommand, command::Command};

  #[test]
  fn execute() {
    let mut light = Light::new();
    let mut light_on_command = LightOnCommand::new(&mut light);
    light_on_command.execute();
    assert_eq!(light.is_on(), true);
  }
}
