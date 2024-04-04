use std::{fmt, ops::{Add, Div, Mul, Sub}};
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

    pub fn to_string(self) -> String {
        format!("FieldElement_{}_{}", self.num, self.prime)
    }

    pub fn pow(&self, power: isize) -> Self {
        let exp = (power % (self.prime - 1).to_isize().unwrap()).to_usize().unwrap();
        let num = pow(self.num, exp) % self.prime;
        FieldElement {
            num,
            prime: self.prime
        }
    }

    // pub fn div(&self, other: &FieldElement) -> FieldElement {
    //     if self.prime != other.prime {
    //         panic!("Can't divide numbers in different fields");
    //     }

    //     let exp = other.num.pow((self.prime - 2) as u32);
    //     let num = (self.num * exp) % self.prime;
    //     println!("exp {}, product: {}", exp, num);
        
    //     FieldElement { num, prime: self.prime }
    // }
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

    fn add(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't add numbers in different fields");
        }

        FieldElement {
            num: (self.num + other.num) % self.prime,
            prime: self.prime
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: FieldElement) -> FieldElement {
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

        FieldElement {
            num,
            prime: self.prime
        }
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't multiply numbers in different fields");
        }

        FieldElement {
            num: (self.num * other.num) % self.prime,
            prime: self.prime
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't divide numbers in different fields");
        }

        let exp = other.num.pow((self.prime - 2) as u32);
        let num = (self.num * exp) % self.prime;
        println!("exp {}, product: {}", exp, num);
        
        FieldElement { num, prime: self.prime }
    }
}

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

    // #[test]
    // fn div_works() {
    //     let prime = 223;
        
    //     let a = FieldElement::new(12, prime);
    //     let b = FieldElement::new(222, prime);
    //     let c = FieldElement::new(211, prime);
        
    //     assert_eq!(a / b, c);
    // }
}