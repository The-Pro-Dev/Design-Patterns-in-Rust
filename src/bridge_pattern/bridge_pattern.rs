use crate::bridge_pattern::bombay_cat::BOMBAY_CAT;
use crate::bridge_pattern::cat;
use crate::bridge_pattern::siamese_cat::SIAMESE_CAT;

pub fn bridge_pattern() {
  println!("** Bridge Pattern **");

	let bombay = cat::know_cat(BOMBAY_CAT);
  println!("{}", bombay);

	let siamese = cat::know_cat(SIAMESE_CAT);
  println!("{}", siamese);
}
