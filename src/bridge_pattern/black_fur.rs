use once_cell::sync::Lazy;

use super::fur::Fur;

pub const BLACK_FUR: Lazy<Fur> = Lazy::new(|| {
  Fur::new("Black")
});
