use super::medium_fry_bucket::MediumFryBucket;

pub fn facade_pattern() {
  println!("** Facade Pattern **");

  let medium_fry_bucket = MediumFryBucket::new();
  medium_fry_bucket.make();

  println!();
}
