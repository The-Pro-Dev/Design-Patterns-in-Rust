use super::{oven::Oven, potato::Potato, bucket::Bucket};

pub struct MediumFryBucket {}

impl MediumFryBucket {
	pub fn new() -> MediumFryBucket {
		MediumFryBucket {  }
	}

	pub fn make(&self) {
		let oven = Oven::new();

		let potato = Potato::new();
		potato.crush();

		oven.turn_on();
		potato.fry();
		oven.turn_off();

		let bucket = Bucket::new();
		bucket.fill(&potato);
		bucket.pack();
	}
}
