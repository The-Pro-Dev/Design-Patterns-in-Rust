use once_cell::sync::Lazy;

use crate::bridge_pattern::gray_fur::GRAY_WHITE_FUR;
use crate::bridge_pattern::cat::Cat;

pub const SIAMESE_CAT: Lazy<Cat> = Lazy::new(|| {
  Cat::new("Ursula", "Siamese", GRAY_WHITE_FUR)
});
