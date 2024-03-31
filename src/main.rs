mod finite_field;

use finite_field::FieldElement;
fn main() {
    let a = FieldElement::new(7, 13);
    let b = FieldElement::new(6, 13);

    println!("{}", a.is_equal(&b));
    println!("{}", a.is_equal(&a));
}
