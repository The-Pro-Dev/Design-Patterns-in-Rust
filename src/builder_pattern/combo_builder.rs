use super::{combo::Combo, hot_wings::HotWings, medium_fries::MediumFries};

pub fn build_friendship_bucket() -> Combo {
  let mut combo = Combo::new("Friendship Bucket");

  combo.add_item(HotWings::new());
  combo.add_item(HotWings::new());
  combo.add_item(MediumFries::new());

  combo
}

pub fn build_vegan_bucket() -> Combo {
  let mut combo = Combo::new("Vegan Bucket");

  combo.add_item(MediumFries::new());
  combo.add_item(MediumFries::new());

  combo
}
