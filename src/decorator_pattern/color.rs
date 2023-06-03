use super::{material::Material, wood::Wood};

pub struct Color {
  color: String,
  base: Wood,
	rate: f64
}

impl Material for Color {
  fn get_cost(&self, area: f64) -> f64 {
    self.rate * area + self.base.get_cost(area)
  }
}

impl Color {
  pub fn new(color: &str, base: Wood, rate: f64) -> Color {
    Color { color: String::from(color), base, rate }
  }

  pub fn get_name(&self) -> String {
    self.base.get_name()
  }

  pub fn get_color(&self) -> String {
    self.color.to_owned()
  }
}

pub fn with_color(color: &str, rate: f64) -> impl Fn(Wood) -> Color + '_ {
  move |base: Wood| {
    Color::new(color, base, rate)
  }
}
