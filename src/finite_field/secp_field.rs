#![allow(unused)]

use std::{fmt, ops::{Add, Deref, Div, Mul, Sub}};
use num::{BigInt, BigUint, FromPrimitive, One, Zero, ToPrimitive, pow};

#[derive(Debug, Clone)]
pub struct S256Field {
    num: BigUint, 
    prime: BigUint
}

impl S256Field {
    pub fn new(num: BigUint) -> Self {
        let secp256k1_prime =
            BigUint::from(2u64).pow(256) - BigUint::from(2u64).pow(32) - BigUint::from(977u64);
        if num >= secp256k1_prime {
            panic!("Num {} not in field range", num);
        }
        Self {num, prime: secp256k1_prime}
    }

    pub fn pow(&self, power: BigUint) -> Self {
        let exp = power % (&self.prime - BigUint::from_u64(1u64).unwrap());
        let num = Self::mod_pow(self.num.clone(), exp.into(), &self.prime);
        // let mut exp = power;
        // while exp < BigUint::from(0u8) {
        //     exp += (self.prime - BigUint::from(1u8));
        // }
        // let num = pow(self.num, exp.into()) % self.prime;
        Self::new(num)
    }

     // credit to https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
     fn mod_pow(mut base: BigUint, mut exp: BigUint, modulus: &BigUint) -> BigUint {
        if *modulus == BigUint::one() {
            return BigUint::zero();
        }
        let mut result = BigUint::one();
        base = base % modulus;
        while exp > BigUint::zero() {
            if &exp % BigUint::from_u64(2u64).unwrap() == BigUint::one() {
                result = result * &base % modulus;
            }
            exp = exp >> 1;
            base = base.clone() * base % modulus
        }
        result
    }

}

impl PartialEq for S256Field {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num && self.prime == other.prime;
    }
}
impl Eq for S256Field {}

impl fmt::Display for S256Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({}))", self.prime, self.num)
    }
}


impl Add for S256Field {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Can't add numbers in different fields");
        }

        Self::new((self.num + other.num) % self.prime)
    }
}

impl Sub for S256Field {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Can't subtract numbers in different fields");
        }

        let num;
        if self.num < other.num {
            let temp_num = (other.num - self.num) % self.prime.clone();
            num = self.prime - temp_num;
        } else {
            num = (self.num - other.num) % self.prime;
        }

        Self::new(num)
    }
}

impl Mul for S256Field {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Can't multiply numbers in different fields");
        }

        Self::new((self.num * other.num) % self.prime)
    }
}

impl Mul<usize> for S256Field {
    type Output = Self;

    fn mul(self, other: usize) -> Self {
        Self::new((self.num * other) % self.prime)
    }
}

impl Mul<S256Field> for usize {
    type Output = S256Field;

    fn mul(self, other: S256Field) -> Self::Output {
        Self::Output::new((self * other.num) % other.prime)
    }
}

impl Div for S256Field {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Can't divide numbers in different fields");
        }

        let exp = S256Field::mod_pow(other.num,self.prime.clone() - BigUint::from_u64(2u64).unwrap(), &self.prime);
        let num = (self.num * exp) % self.prime;
        
        Self::new(num)
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lib_works() {
        let a = S256Field::new(BigUint::from(12u64));
        let b = S256Field::new(BigUint::from(222u64));
        let c = S256Field::new(BigUint::from(11u64));
        
        assert_eq!(a+b, c);
    }
}