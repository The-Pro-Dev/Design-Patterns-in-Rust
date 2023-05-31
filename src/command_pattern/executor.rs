use super::command::Command;

pub struct Executor {}

impl Executor {
  pub fn new() -> Executor {
    Executor { }
  }

  pub fn execute(&self, mut command: impl Command) {
    command.execute();
  }
}
