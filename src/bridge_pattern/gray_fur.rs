use once_cell::sync::Lazy;

use crate::bridge_pattern::fur::Fur;

pub const GRAY_WHITE_FUR: Lazy<Fur> = Lazy::new(|| {
  Fur::new("Gray and White")
});
