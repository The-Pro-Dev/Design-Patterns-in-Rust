pub trait Material {
  fn get_cost(self: &Self, area: f64) -> f64;
}
