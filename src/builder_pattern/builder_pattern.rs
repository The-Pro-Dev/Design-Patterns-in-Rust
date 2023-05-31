use super::{combo::details, combo_builder::{build_friendship_bucket, build_vegan_bucket}, coupon::Coupon};

pub fn builder_pattern() {
  println!("** Builder Pattern **");

  let coupon10pctoff = Coupon::percent("Hot Wings Special (10% off over ₹100)", 0.1, Some(100.0));
  let mut friendship_bucket = build_friendship_bucket();
  friendship_bucket.apply_coupon(coupon10pctoff);
  let friendship_bucket_details = details(&friendship_bucket);
  println!("{} [on applying '{}']", friendship_bucket_details, friendship_bucket.get_coupon_code());

  let vegan_bucket = build_vegan_bucket();
  let vegan_bucket_details = details(&vegan_bucket);
  println!("{}", vegan_bucket_details);

  println!()
}
