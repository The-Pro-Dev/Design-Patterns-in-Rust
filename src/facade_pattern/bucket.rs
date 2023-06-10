use super::bucket_item::BucketItem;

pub struct Bucket {}

impl Bucket {
  pub fn new() -> Bucket {
    Bucket {  }
  }

  pub fn fill(&self, item: &impl BucketItem) {
    println!("Filling bucket with {}.", item.name())
  }

  pub fn pack(&self) {
    println!("Packed and ready to ship.")
  }
}
