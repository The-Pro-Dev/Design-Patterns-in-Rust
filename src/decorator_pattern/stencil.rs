use super::{material::Material, color::Color};

pub struct Stencil {
  code: String,
  color: Color,
	rate: f64
}

impl Material for Stencil {
  fn get_cost(&self, area: f64) -> f64 {
    self.rate * area + self.color.get_cost(area)
  }
}

impl Stencil {
  pub fn new(code: &str, color: Color, rate: f64) -> Stencil {
    Stencil { code: String::from(code), color, rate }
  }

  pub fn get_name(&self) -> String {
    self.color.get_name()
  }

  pub fn get_color(&self) -> String {
    self.color.get_color()
  }

  pub fn get_code(&self) -> String {
    self.code.to_owned()
  }
}

pub fn with_stencil(code: &str, rate: f64) -> impl Fn(Color) -> Stencil + '_ {
  move |color: Color| {
    Stencil::new(code, color, rate)
  }
}
