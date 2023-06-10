use super::bucket_item::BucketItem;

pub struct Potato {}

impl Potato {
  pub fn new() -> Potato {
    Potato {  }
  }

  pub fn crush(&self) {
    println!("Crushing Potato")
  }

  pub fn fry(&self) {
    println!("Frying Potato")
  }
}

impl BucketItem for Potato {
  fn name(&self) -> &str {
    "Potato"
  }
}
