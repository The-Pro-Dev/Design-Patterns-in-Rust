use super::food_item::FoodItem;

pub struct HotWings {}

impl HotWings {
  pub fn new() -> HotWings {
    HotWings { }
  }
}

impl FoodItem for HotWings {
  fn get_name(&self) -> String {
    String::from("Hot Wings")
  }

  fn get_price(&self) -> f64 {
    170.00
  }
}
