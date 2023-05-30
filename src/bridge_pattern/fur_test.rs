#[cfg(test)]
mod fur_tests {
  use crate::bridge_pattern::fur::Fur;

  #[test]
  fn fur_get_colors() {
    let fur = Fur::new("White");
    assert_eq!(fur.get_colors(), "White");
  }
}
