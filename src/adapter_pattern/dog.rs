use crate::adapter_pattern::animal::AnimalAdapter;

pub struct Dog {
	pub name: String
}

impl Dog {
  pub fn bark() -> String {
    String::from("Bark!")
  }

  pub fn name(&self) -> String {
    self.name.to_owned()
  }
}

pub struct DogAdapter {
	pub dog: Dog
}

impl AnimalAdapter for DogAdapter {
  fn name(&self) -> String {
    Dog::name(&self.dog)
  }

  fn kind(&self) -> String {
    String::from("dog")
  }

  fn sound(&self) -> String {
    Dog::bark()
  }
}
