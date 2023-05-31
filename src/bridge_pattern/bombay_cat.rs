use once_cell::sync::Lazy;

use super::{black_fur::BLACK_FUR, cat::Cat};

pub const BOMBAY_CAT: Lazy<Cat> = Lazy::new(|| {
  Cat::new("Simon", "Bombay", BLACK_FUR)
});
