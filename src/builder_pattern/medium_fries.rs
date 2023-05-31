use super::food_item::FoodItem;

pub struct MediumFries {}

impl MediumFries {
  pub fn new() -> MediumFries {
    MediumFries {  }
  }
}

impl FoodItem for MediumFries {
  fn get_name(&self) -> String {
    String::from("Medium Fries")
  }

  fn get_price(&self) -> f64 {
    90.00
  }
}
