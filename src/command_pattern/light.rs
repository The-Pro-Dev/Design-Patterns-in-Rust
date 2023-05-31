#[derive(PartialEq)]
enum LightState {
  On,
  Off
}

pub struct Light {
  state: LightState,
}

impl Light {
  pub fn new() -> Light {
    Light { state: LightState::On }
  }

  pub fn turn_on(&mut self) {
    self.state = LightState::On;
  }

  pub fn turn_off(&mut self) {
    self.state = LightState::Off;
  }

  pub fn is_on(&self) -> bool {
    self.state == LightState::On
  }
}

pub fn status(light: &Light) -> String {
  String::from(format!("The light is {}.", if light.is_on() { "On" } else { "Off" }))
}
