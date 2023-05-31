use crate::command_pattern::{light::{Light, status}, light_off_command::LightOffCommand, light_on_command::LightOnCommand, executor::Executor};

pub fn command_pattern() {
  println!("** Command Pattern **");

  let mut light = Light::new();
  println!("{}", status(&light));

  let executor =  Executor::new();

  let light_off_command = LightOffCommand::new(&mut light);
  executor.execute(light_off_command);
  println!("{}", status(&light));

  let light_on_command = LightOnCommand::new(&mut light);
  executor.execute(light_on_command);
  println!("{}", status(&light));

  println!();
}
