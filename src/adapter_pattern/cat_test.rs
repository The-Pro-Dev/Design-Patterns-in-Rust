#[cfg(test)]
mod cat_test {
  use crate::adapter_pattern::animal::AnimalAdapter;
  use crate::adapter_pattern::cat::{Cat, CatAdapter};

  #[test]
  fn cat_meow() {
    let cat = Cat{name: String::from("Tom")};
    assert_eq!(cat.meow(), String::from("Meow!"));
  }

  #[test]
  fn cat_name() {
    let cat = Cat{name: String::from("Tom")};
    assert_eq!(cat.name(), String::from("Tom"));
  }

  #[test]
  fn cat_adapter_name() {
    let cat = Cat{name: String::from("Tom")};
    let cat_adapter = CatAdapter{cat};
    assert_eq!(cat_adapter.name(), String::from("Tom"));
  }

  #[test]
  fn cat_adapter_kind() {
    let cat = Cat{name: String::from("Tom")};
    let cat_adapter = CatAdapter{cat};
    assert_eq!(cat_adapter.kind(), String::from("cat"));
  }

  #[test]
  fn cat_adapter_sound() {
    let cat = Cat{name: String::from("Tom")};
    let cat_adapter = CatAdapter{cat};
    assert_eq!(cat_adapter.sound(), String::from("Meow!"));
  }
}
