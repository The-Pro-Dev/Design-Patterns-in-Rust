use once_cell::sync::Lazy;

use super::fur::Fur;

pub struct Cat {
	name: String,
	breed: String,
	fur: Lazy<Fur>
}

impl Cat {
  pub fn new(name: &str, breed: &str, fur: Lazy<Fur>) -> Cat {
    Cat{ name: String::from(name), breed: String::from(breed), fur }
  }

  pub fn get_name(&self) -> String {
    return self.name.to_owned()
  }

  pub fn get_breed(&self) -> String {
    return self.breed.to_owned()
  }

  pub fn get_colors(&self) -> String {
    return self.fur.get_colors()
  }
}

pub fn know_cat(cat: Lazy<Cat>) -> String {
  String::from(std::format!("{} is a {} cat and has a fur coat of color(s) {}", cat.get_name(), cat.get_breed(), cat.get_colors()))
}
