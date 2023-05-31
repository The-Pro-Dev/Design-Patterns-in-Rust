#[cfg(test)]
mod coupon_test {
  use crate::builder_pattern::coupon::Coupon;

  #[test]
  fn flat() {
    let coupon = Coupon::flat("abcd", 10.0, None);
    assert_eq!(coupon.apply_discount(30.0), 20.0);
  }

  #[test]
  fn percent() {
    let coupon = Coupon::percent("abcd", 10.0, None);
    assert_eq!(coupon.apply_discount(30.0), 27.0);
  }

  #[test]
  fn get_code() {
    let coupon = Coupon::flat("abcd", 10.0, None);
    assert_eq!(coupon.get_code(), "abcd");
  }

  #[test]
  fn apply_discount() {
    let coupon = Coupon::flat("abcd", 10.0, Some(100.0));
    assert_eq!(coupon.apply_discount(30.0), 30.0);
  }
}
