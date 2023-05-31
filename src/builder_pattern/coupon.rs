#[derive(PartialEq)]
enum CouponKind {
  FlatDiscount,
  PercentDiscount
}

pub struct Coupon {
  code: String,
  kind: CouponKind,
  discount: f64,
  min_value: f64
}

#[allow(dead_code)]
impl Coupon {
  fn new(code: &str, kind: CouponKind, discount: f64, min_value: f64) -> Coupon {
    Coupon { code: String::from(code), kind, discount, min_value }
  }

  pub fn flat(code: &str, discount: f64, min_value: Option<f64>) -> Coupon {
    Self::new(code, CouponKind::FlatDiscount, discount, min_value.unwrap_or(0.0))
  }

  pub fn percent(code: &str, discount: f64, min_value: Option<f64>) -> Coupon {
    Self::new(code, CouponKind::PercentDiscount, discount, min_value.unwrap_or(0.0))
  }

  pub fn get_code(&self) -> String {
    self.code.to_owned()
  }

  pub fn apply_discount(&self, price: f64) -> f64 {
    if self.min_value > price {
      return price;
    }

    if self.kind == CouponKind::FlatDiscount {
      return price - self.discount;
    }

    if self.kind == CouponKind::PercentDiscount {
      return price * (1.0 - self.discount * 0.01);
    }

    panic!("Undefined behaviour")
  }
}
