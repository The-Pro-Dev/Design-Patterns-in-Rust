use super::animal::AnimalAdapter;

pub struct Dog {
	name: String
}

impl Dog {
  pub fn new(name: &str) -> Dog {
    Dog{name: String::from(name)}
  }

  pub fn bark(&self) -> String {
    String::from("Bark!")
  }

  pub fn name(&self) -> String {
    self.name.to_owned()
  }
}

pub struct DogAdapter {
	dog: Dog
}

impl DogAdapter {
  pub fn new(dog: Dog) -> DogAdapter {
    DogAdapter { dog }
  }
}

impl AnimalAdapter for DogAdapter {
  fn name(&self) -> String {
    Dog::name(&self.dog)
  }

  fn kind(&self) -> String {
    String::from("dog")
  }

  fn sound(&self) -> String {
    Dog::bark(&self.dog)
  }
}
