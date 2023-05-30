use once_cell::sync::Lazy;

use crate::bridge_pattern::black_fur::BLACK_FUR;
use crate::bridge_pattern::cat::Cat;

pub const BOMBAY_CAT: Lazy<Cat> = Lazy::new(|| {
  Cat::new("Simon", "Bombay", BLACK_FUR)
});
