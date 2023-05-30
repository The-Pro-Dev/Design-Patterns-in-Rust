use crate::adapter_pattern::animal::AnimalAdapter;

pub struct Cat {
	pub name: String
}

impl Cat {
  pub fn meow(&self) -> String {
    String::from("Meow!")
  }

  pub fn name(&self) -> String {
    self.name.to_owned()
  }
}

pub struct CatAdapter {
	pub cat: Cat
}

impl AnimalAdapter for CatAdapter {
  fn name(&self) -> String {
    Cat::name(&self.cat)
  }

  fn kind(&self) -> String {
    String::from("cat")
  }

  fn sound(&self) -> String {
    Cat::meow(&self.cat)
  }
}
