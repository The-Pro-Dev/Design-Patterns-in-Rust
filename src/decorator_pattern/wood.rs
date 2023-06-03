use super::material::Material;

pub struct Wood {
  name: String,
	rate: f64
}

impl Material for Wood {
  fn get_cost(&self, area: f64) -> f64 {
    self.rate * area
  }
}

impl Wood {
  pub fn new(name: &str, rate: f64) -> Wood {
    Wood { name: String::from(name), rate }
  }

  pub fn get_name(&self) -> String {
    self.name.to_owned()
  }
}
