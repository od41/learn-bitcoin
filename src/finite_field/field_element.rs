use std::ops::{Add, Div, Mul, Sub};
use num::pow;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FieldElement {
    num: u128, 
    prime: u128
}

impl FieldElement {
    pub fn new(num: u128, prime: u128) -> FieldElement {
        if num >= prime {
            panic!("Num not in field range 0 to {}", prime);
        }
        FieldElement {num, prime}
    }

    pub fn to_string(self) -> String {
        format!("FieldElement_{}_{}", self.num, self.prime)
    }

    pub fn eq(&self, other: &FieldElement) -> bool {
        self.num == other.num && self.prime == other.prime
    }

    pub fn ne(&self, other: &FieldElement) -> bool {
        self.num != other.num || self.prime != other.prime
    }

    pub fn add(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't add numbers in different fields");
        }

        FieldElement {
            num: (self.num + other.num) % self.prime,
            prime: self.prime
        }
    }

    pub fn sub(&self, other: &FieldElement) -> FieldElement {
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

    pub fn mul(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't multiply numbers in different fields");
        }

        FieldElement {
            num: (self.num * other.num) % self.prime,
            prime: self.prime
        }
    }

    pub fn s_mul(&self, scalar: u64) -> FieldElement {
        FieldElement {
            num: (self.num * scalar as u128) % self.prime,
            prime: self.prime
        }
    }

    pub fn pow(&self, power: usize) -> FieldElement {
        let num = pow(self.num, power) % self.prime;
        FieldElement {
            num,
            prime: self.prime
        }
    }

    pub fn div(&self, other: &FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Can't divide numbers in different fields");
        }

        // # use Fermat's little theorem:
        // # self.num**(p-1) % p == 1
        // # this means:
        // # 1/n == pow(n, p-2, p)
        // # we return an element of the same class
        // num = self.num * pow(other.num, self.prime - 2, self.prime) % self.prime
        // return self.__class__(num, self.prime

        let exp = other.num.pow((self.prime - 2) as u32);
        let num = (self.num * exp) % self.prime;
        println!("exp {}, product: {}", exp, num);
        
        FieldElement { num, prime: self.prime }
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