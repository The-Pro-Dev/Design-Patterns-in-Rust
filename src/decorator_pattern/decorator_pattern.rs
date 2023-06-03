use super::{wood::Wood, color::with_color, stencil::with_stencil, material::Material};

pub fn decorator_pattern() {
  println!("** Decorator Pattern **");

  let wood = Wood::new("Ply Wood", 12.00);
  let wood_with_color = with_color("Black", 8.50)(wood);
  let wood_with_color_and_stencil = with_stencil("Birdie", 3.0)(wood_with_color);

  let cost = wood_with_color_and_stencil.get_cost(12.0 * 11.0);
  println!(
    "Cost of {} product [12 x 11] with {} paint and {} stencil is: ₹{}",
    wood_with_color_and_stencil.get_name(),
    wood_with_color_and_stencil.get_color(),
    wood_with_color_and_stencil.get_code(),
    cost
  );

  println!();
}
