use super::animal::AnimalAdapter;

pub struct Cat {
	name: String
}

impl Cat {
  pub fn new(name: &str) -> Cat {
    Cat{name: String::from(name)}
  }

  pub fn meow(&self) -> String {
    String::from("Meow!")
  }

  pub fn name(&self) -> String {
    self.name.to_owned()
  }
}

pub struct CatAdapter {
	cat: Cat
}

impl CatAdapter {
  pub fn new(cat: Cat) -> CatAdapter {
    CatAdapter { cat }
  }
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
