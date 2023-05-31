pub trait Command {
  fn execute(self: &mut Self);
}
