use super::{light::Light, command::Command};

pub struct LightOffCommand<'a> {
  light: &'a mut Light
}

impl Command for LightOffCommand<'_> {
  fn execute(self: &mut Self) {
      self.light.turn_off();
  }
}

impl LightOffCommand<'_> {
  pub fn new(light: &mut Light) -> LightOffCommand {
    LightOffCommand { light }
  }
}
