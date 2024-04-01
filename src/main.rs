mod finite_field;

use std::env;
use finite_field::field_element::FieldElement;
use finite_field::elliptic_curve::Point;
fn main() {
    // this method needs to be inside main() method
  env::set_var("RUST_BACKTRACE", "full");

    let a = FieldElement::new(0, 223);
    let b = FieldElement::new(7, 223);
    let x = FieldElement::new(192, 223);
    let y = FieldElement::new(105, 223);
    
    let p1 = Point::new(Some(x), Some(y), a, b);

}
