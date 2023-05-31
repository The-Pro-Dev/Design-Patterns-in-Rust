use super::{light::Light, command::Command};

pub struct LightOnCommand<'a> {
  light: &'a mut Light
}

impl Command for LightOnCommand<'_> {
  fn execute(self: &mut Self) {
      self.light.turn_on();
  }
}

impl LightOnCommand<'_> {
  pub fn new(light: &mut Light) -> LightOnCommand {
    LightOnCommand { light }
  }
}
