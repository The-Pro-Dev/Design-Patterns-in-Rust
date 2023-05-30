pub struct Fur {
	colors: String
}

impl Fur {
  pub fn new(colors: &str) -> Fur {
    Fur { colors: String::from(colors) }
  }

  pub fn get_colors(&self) -> String {
    return self.colors.to_owned()
  }
}
