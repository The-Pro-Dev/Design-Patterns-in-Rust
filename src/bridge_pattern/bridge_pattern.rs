use super::{bombay_cat::BOMBAY_CAT, cat::know_cat, siamese_cat::SIAMESE_CAT};

pub fn bridge_pattern() {
  println!("** Bridge Pattern **");

	let bombay = know_cat(BOMBAY_CAT);
  println!("{}", bombay);

	let siamese = know_cat(SIAMESE_CAT);
  println!("{}", siamese);

  println!()
}
