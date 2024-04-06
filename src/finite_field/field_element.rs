#![allow(unused)]

use std::{fmt, ops::{Add, Deref, Div, Mul, Sub}};
use num::{pow, ToPrimitive};

#[derive(Debug, Clone, Copy)]
pub struct FieldElement {
    num: usize, 
    prime: usize
}

impl FieldElement {
    pub fn new(num: usize, prime: usize) -> FieldElement {
        if num >= prime {
            panic!("Num not in field range 0 to {}", prime);
        }
        FieldElement {num, prime}
    }

    pub fn pow(&self, power: isize) -> Self {
        let mut exp = power;
        while exp < 0 {
            exp += (self.prime - 1).to_isize().unwrap();
        }
        let num = pow(self.num, exp.to_usize().unwrap()) % self.prime;
        FieldElement {
            num,
            prime: self.prime
        }
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num && self.prime == other.prime;
    }
}
impl Eq for FieldElement {}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({}))", self.prime, self.num)
    }
}


impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Can't add numbers in different fields");
        }

        Self {
            num: (self.num + other.num) % self.prime,
            prime: self.prime
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Can't subtract numbers in different fields");
        }

        let num;
        if self.num < other.num {
            let temp_num = (other.num - self.num) % self.prime;
            num = self.prime - temp_num;
        } else {
            num = (self.num - other.num) % self.prime;
        }

        Self {
            num,
            prime: self.prime
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Can't multiply numbers in different fields");
        }

        Self {
            num: (self.num * other.num) % self.prime,
            prime: self.prime
        }
    }
}

impl Mul<usize> for FieldElement {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        Self {
            num: (self.num * other) % self.prime,
            prime: self.prime
        }
    }
}

impl Mul<FieldElement> for usize {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> Self::Output {
        Self::Output {
            num: (self * other.num) % other.prime,
            prime: other.prime
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Can't divide numbers in different fields");
        }

        let exp = mod_exp::mod_exp(other.num,self.prime - 2, self.prime);
        let num = (self.num * exp) % self.prime;
        
        Self { num, prime: self.prime }
    }
}

#[derive(Debug)]
pub struct S256Field {
    num: usize,
}

impl S256Field {
    pub fn new(num: usize) -> Self {
        // Use the specific prime value for S256Field
        Self { num }
    }
}

// Derive traits from FieldElement to S256Field
impl From<S256Field> for FieldElement {
    fn from(s256: S256Field) -> Self {
        let prime: usize = pow(2, 256) - pow(2, 32) - 977;
        FieldElement::new(s256.num, prime) // Use the S256 prime value
    }
}

// For easier conversion from S256Field to FieldElement
// impl Deref for S256Field {
//     type Target = FieldElement;

//     fn deref(&self) -> &Self::Target {
//         // Convert S256Field to FieldElement when needed
//         // by using the specific prime value
//         let temp = Self::Target{num: self.num, prime: 256};
//         temp.clone()
//     }
// }


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let prime = 223;
        
        let a = FieldElement::new(12, prime);
        let b = FieldElement::new(222, prime);
        let c = FieldElement::new(11, prime);
        
        assert_eq!(a+b, c);
    }

    #[test]
    fn sub_works() {
        let prime = 223;
        
        let a = FieldElement::new(12, prime);
        let b = FieldElement::new(222, prime);
        let c = FieldElement::new(13, prime);
        
        assert_eq!(a-b, c);
    }

    #[test]
    fn mul_works() {
        let prime = 223;
        
        let a = FieldElement::new(12, prime);
        let b = FieldElement::new(222, prime);
        let c = FieldElement::new(211, prime);
        
        assert_eq!(a * b, c);
    }

    #[test]
    fn pow_works() {
        let prime = 13;
        
        let a = FieldElement::new(3, prime);
        let b = FieldElement::new(1, prime);
        assert_eq!(a.pow(3), b);
        
        let a = FieldElement::new(5, 31);
        let b = FieldElement::new(18, 31);
        assert_eq!(a.pow(5) * b, FieldElement::new(16, 31));

        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(8, 13);
        debug_assert_eq!(a.pow(-15), b);
    }

    #[test]
    fn div_works() {
        let prime = 223;
        
        let a = FieldElement::new(12, prime);
        let b = FieldElement::new(222, prime);
        let c = FieldElement::new(211, prime);
        
        assert_eq!(a / b, c);
    }

    #[test]
    fn secp_field_element_works() {        
        let point = S256Field::new(42);
        let p2 = FieldElement::new(10, 256);
        let sum = p2 + point;
        println!("point is: {:?}", point);
    }
}