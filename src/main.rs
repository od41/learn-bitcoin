mod finite_field;

use std::env;
use finite_field::field_element::FieldElement;
use finite_field::elliptic_curve::Point;
fn main() {
    // this method needs to be inside main() method
  env::set_var("RUST_BACKTRACE", "full");

    let a = FieldElement::new(3, 31);
    let b = FieldElement::new(24, 31);

    let c = a.add(&b);
    let d = a.sub(&b);

    let mul = a.mul(&b);
    let exp = a.exp(3);
    // let div = a.div(&b);

    println!("a + b {} ", c.to_string()); // finite field addition
    println!("a - b {} ", d.to_string()); // finite field subtraction

    println!("a * b {} ", mul.to_string()); // finite field multiplication
    println!("a ^ 3 {} ", exp.to_string()); // finite field exponents
    // println!("a / b {} ", div.to_string()); // finite field exponents

}
