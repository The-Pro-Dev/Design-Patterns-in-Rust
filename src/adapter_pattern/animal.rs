pub trait AnimalAdapter {
  fn name(self: &Self) -> String;
  fn kind(self: &Self) -> String;
  fn sound(self: &Self) -> String;
}

pub fn sound(animal_adapter: impl AnimalAdapter) -> String {
  String::from(std::format!("{} is a {} and makes sound of {}", animal_adapter.name(), animal_adapter.kind(), animal_adapter.sound()))
}
