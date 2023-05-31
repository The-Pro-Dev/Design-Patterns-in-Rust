pub trait FoodItem {
  fn get_name(self: &Self) -> String;
  fn get_price(self: &Self) -> f64;
}
