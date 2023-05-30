#[cfg(test)]
mod dog_test {
  use crate::adapter_pattern::animal::AnimalAdapter;
  use crate::adapter_pattern::dog::{Dog, DogAdapter};

  #[test]
  fn dog_meow() {
    let dog = Dog{name: String::from("Bruno")};
    assert_eq!(dog.meow(), String::from("Bark!"));
  }

  #[test]
  fn dog_name() {
    let dog = Dog{name: String::from("Bruno")};
    assert_eq!(dog.name(), String::from("Bruno"));
  }

  #[test]
  fn dog_adapter_name() {
    let dog = Dog{name: String::from("Bruno")};
    let dog_adapter = DogAdapter{dog};
    assert_eq!(dog_adapter.name(), String::from("Bruno"));
  }

  #[test]
  fn dog_adapter_kind() {
    let dog = Dog{name: String::from("Bruno")};
    let dog_adapter = DogAdapter{dog};
    assert_eq!(dog_adapter.kind(), String::from("cat"));
  }

  #[test]
  fn dog_adapter_sound() {
    let dog = Dog{name: String::from("Bruno")};
    let dog_adapter = DogAdapter{dog};
    assert_eq!(dog_adapter.sound(), String::from("Bark!"));
  }
}
