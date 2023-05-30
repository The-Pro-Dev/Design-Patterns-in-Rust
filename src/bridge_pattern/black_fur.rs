use once_cell::sync::Lazy;

use crate::bridge_pattern::fur::Fur;

pub const BLACK_FUR: Lazy<Fur> = Lazy::new(|| {
  Fur::new("Black")
});
