#[cfg(test)]
mod cat_tests {
  use once_cell::sync::Lazy;

  use crate::bridge_pattern::{fur::Fur, cat::Cat};

  const WHITE_FUR: Lazy<Fur> = Lazy::new(|| {
    Fur::new("White")
  });

  #[test]
  fn cat_get_name() {
    let cat = Cat::new("Tom", "Persian", WHITE_FUR);
    assert_eq!(cat.get_name(), "Tom");
  }

  #[test]
  fn cat_get_breed() {
    let cat = Cat::new("Tom", "Persian", WHITE_FUR);
    assert_eq!(cat.get_breed(), "Persian");
  }

  #[test]
  fn cat_get_colors() {
    let cat = Cat::new("Tom", "Persian", WHITE_FUR);
    assert_eq!(cat.get_colors(), "White");
  }
}
