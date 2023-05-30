use crate::adapter_pattern::animal;
use crate::adapter_pattern::cat;
use crate::adapter_pattern::dog;

pub fn adapter_pattern() {
	println!("** Adapter Pattern **");

	let cat = cat::Cat{name: String::from("Evelyn")};
	let cat_adapter = cat::CatAdapter{cat};
	let cat_sound = animal::sound(cat_adapter);
	println!("{}", cat_sound);

	let dog = dog::Dog{name: String::from("Bruno")};
	let dog_adapter = dog::DogAdapter{dog};
	let dog_sound = animal::sound(dog_adapter);
	println!("{}", dog_sound);
}
