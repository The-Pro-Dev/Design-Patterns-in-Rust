#[cfg(test)]
mod combo_test {
  use crate::builder_pattern::{combo::{Combo, NO_COUPON_CODE}, hot_wings::HotWings, food_item::FoodItem, coupon::Coupon};

  #[test]
  fn add_item() {
    let wings = HotWings::new();
    let wings_name = wings.get_name();
    let mut combo = Combo::new("Special");
    combo.add_item(wings);
    assert!(combo.get_items().contains(&wings_name));
  }

  #[test]
  fn get_coupon_code() {
    let combo = Combo::new("Special");
    assert_eq!(combo.get_coupon_code(), String::from(NO_COUPON_CODE))
  }

  #[test]
  fn apply_coupon() {
    let coupon = Coupon::flat("Coupon", 1.0, None);
    let mut combo = Combo::new("Special");
    combo.apply_coupon(coupon);
    assert_eq!(combo.get_coupon_code(), "Coupon");
  }

  #[test]
  fn get_name() {
    let combo = Combo::new("Special");
    assert_eq!(combo.get_name(), "Special");
  }

  #[test]
  fn get_price() {
    let wings = HotWings::new();
    let wings_price = wings.get_price();
    let mut combo = Combo::new("Special");
    combo.add_item(wings);
    assert_eq!(combo.get_price(), wings_price);

    let coupon = Coupon::flat("Coupon", 10.0, None);
    combo.apply_coupon(coupon);
    assert_eq!(combo.get_price(), wings_price - 10.0);
  }
}
