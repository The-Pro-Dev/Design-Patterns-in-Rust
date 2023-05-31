use super::{coupon::Coupon, food_item::FoodItem};

pub const NO_COUPON_CODE: &str = "No Coupon Applied";

pub struct Combo {
  name: String,
  items: Vec<Box<dyn FoodItem>>,
  coupon: Option<Coupon>
}

impl Combo {
  pub fn new(name: &str) -> Combo {
    Combo { name: String::from(name), items: Vec::new(), coupon: None }
  }

  pub fn add_item(&mut self, item: impl FoodItem + 'static) {
    self.items.push(Box::new(item));
  }

  pub fn apply_coupon(&mut self, coupon: Coupon) {
    self.coupon = Some(coupon);
  }

  pub fn get_name(&self) -> String {
    self.name.to_owned()
  }

  pub fn get_coupon_code(&self) -> String {
    if self.coupon.is_some() {
      return self.coupon.as_ref().unwrap().get_code()
    }

    String::from(NO_COUPON_CODE)
  }

  pub fn get_items(&self) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    for item in self.items.iter() {
      v.push(item.get_name())
    }

    v
  }

  pub fn get_price(&self) -> f64 {
    let mut price = 0.0;

    for item in self.items.iter() {
      price += item.get_price();
    }

    if self.coupon.is_some() {
      price = self.coupon.as_ref().unwrap().apply_discount(price);
    }

    price
  }
}

pub fn details(combo: &Combo) -> String {
  String::from(std::format!("{} contains ({}) and costs ₹{}", combo.get_name(), combo.get_items().join(", "), combo.get_price()))
}
