use once_cell::sync::Lazy;

use super::{cat::Cat, gray_fur::GRAY_WHITE_FUR};

pub const SIAMESE_CAT: Lazy<Cat> = Lazy::new(|| {
  Cat::new("Ursula", "Siamese", GRAY_WHITE_FUR)
});
