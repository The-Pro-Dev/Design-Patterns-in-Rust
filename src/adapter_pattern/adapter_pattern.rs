use crate::adapter_pattern::animal;
use crate::adapter_pattern::cat;
use crate::adapter_pattern::dog;

pub fn adapter_pattern() {
	println!("** Adapter Pattern **");

	let cat = cat::Cat::new("Evelyn");
	let cat_adapter = cat::CatAdapter::new(cat);
	let cat_sound = animal::sound(cat_adapter);
	println!("{}", cat_sound);

	let dog = dog::Dog::new("Bruno");
	let dog_adapter = dog::DogAdapter::new(dog);
	let dog_sound = animal::sound(dog_adapter);
	println!("{}", dog_sound);
}
