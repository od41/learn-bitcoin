#![allow(unused)]
use num::{pow, BigInt, BigUint, FromPrimitive, Num, One, ToPrimitive, Zero};

use super::secp_field::S256Field;
use std::ops::{Add, Mul, BitAnd};

#[derive(Debug, Clone)]
pub struct S256Point {
    a: S256Field,
    b: S256Field,
    x: Option<S256Field>,
    y: Option<S256Field>,
}

impl S256Point {
    pub fn new(x: Option<S256Field>, y: Option<S256Field>) -> S256Point {
        let a = S256Field::new(BigUint::from(0u64));
        let b = S256Field::new(BigUint::from(7u64));
        
        if x == None || y == None {
            return S256Point {a, b, x: None, y: None};
        }
        let x = x.unwrap();
        let y = y.unwrap();
        if y.clone().pow(BigUint::from(2u64)) != x.clone().pow(BigUint::from(3u64)) + (a.clone() * x.clone()) + b.clone() {
            panic!("Value ({:?} {:?}) is not on the curve", x, y);
        }
        S256Point {a, b, x: Some(x), y: Some(y)}
    }  

    pub fn infinity_point() -> S256Point {
        S256Point::new(None, None)
    }  

    pub fn generator() -> S256Point {
        let gx =  BigUint::from_str_radix(
            "79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
            16,
        ).unwrap();
        let gy =  BigUint::from_str_radix(
            "483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
            16,
        ).unwrap();
        
        let x = S256Field::new(gx);
        let y = S256Field::new(gy);
        S256Point::new(Some(x), Some(y))
    }
}

impl PartialEq for S256Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &S256Point) -> bool {
        self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b
    }
}
impl Eq for S256Point {}

impl Add for S256Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        if self.x == None || self.y == None {
            // return other
            S256Point{
                x: other.x,
                y: other.y,
                a: other.a,
                b: other.b
            }
        } else if other.x == None || other.y == None {
            // return self
            S256Point{
                x: self.x,
                y: self.y,
                a: self.a,
                b: self.b
            }
        } else if self == other && (self.y.clone().unwrap() == 0 * self.x.clone().unwrap() ){
            // if the line is a tangent to the curve and y = 0
            // return the identity (i.e)
            S256Point{
                x: None,
                y: None,
                a: self.a,
                b: self.b
            }
        } else if self.x == other.x && self.y !=  other.y {
            // returns a S256Point at infinity...it's a vertical line
            S256Point{
                x: None,
                y: None,
                a: self.a,
                b: self.b
            }
        } else if self.x != other.x {
            // x1 != x2
            let x1 = self.x.unwrap();
            let y1 = self.y.unwrap();
            let x2 = other.x.unwrap();
            let y2 = other.y.unwrap();

            let slope = (y2 - y1.clone()) / (x2.clone() - x1.clone());

            let x3 = slope.pow(BigUint::from(2u64)) - x1.clone() - x2;
            let y3 = (slope * (x1 - x3.clone())) - y1;

            S256Point{
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b
            }
        } else if self.x == other.x && self.y == other.y {
            // they are at the same S256Point and you calculate 
            // the slope of the tangent to that S256Point
            let x = self.x.unwrap();
            let y = self.y.unwrap();

            let slope = ((3 * x.clone().pow(BigUint::from(2u64))) + self.a.clone()) / (2 * y.clone());

            let x3 = slope.pow(BigUint::from(2u64)) - (2 * x.clone());
            let y3 = (slope * (x - x3.clone())) - y;


            S256Point{
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b
            }
        } else {
            S256Point{
                x: self.x,
                y: self.y,
                a: self.a,
                b: self.b
            }
        }
    }
}

// S256Point * usize
impl Mul<usize> for S256Point {
    type Output = Self;

    fn mul(self, coefficient: usize) -> Self {
        let mut product = S256Point {x: None, y: None, a: self.a.clone(), b: self.b.clone()};
        let mut coef = coefficient.clone();
        let mut current = self.clone();
        
        while coef > 0 {
            if coef & 1 == 1 {
                product = product + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        
        product
    }
}

// usize * S256Point
impl Mul<S256Point> for usize {
    type Output = S256Point;

    fn mul(self, other: S256Point) -> Self::Output {
        let mut product = S256Point {x: None, y: None, a: other.a.clone(), b: other.b.clone()};
        let mut coef = self.clone();
        let mut current = other.clone();
        
        while coef > 0 {
            if coef & 1 == 1 {
                product = product + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        
        product
    }
}

// S256Point * BigUint
impl Mul<BigUint> for S256Point {
    type Output = Self;

    fn mul(self, coefficient: BigUint) -> Self {
        let mut product = S256Point {x: None, y: None, a: self.a.clone(), b: self.b.clone()};
        let mut coef = coefficient.clone();
        let mut current = self.clone();
        
        while coef > S256Field::zero() {
            if coef.clone().bitand(S256Field::one()) == S256Field::one() {
                product = product + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        
        product
    }
}

// BigUint * S256Point
impl Mul<S256Point> for BigUint {
    type Output = S256Point;

    fn mul(self, other: S256Point) -> Self::Output {
        let mut product = S256Point {x: None, y: None, a: other.a.clone(), b: other.b.clone()};
        let mut coef = self.clone();
        let mut current = other.clone();
        
        while coef > S256Field::zero() {
            if coef.clone().bitand(S256Field::one()) == S256Field::one() {
                product = product + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        
        product
    }
}


#[cfg(test)]
pub mod tests {
    use num::Num;

    use super::*;

    #[test]
    fn is_valid_curves() {
        // TODO:
    }

    #[test]
    fn exercise_works() {
        let z =  BigUint::from_str_radix(
            "bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
            16,
        ).unwrap();
        let r =  BigUint::from_str_radix(
            "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        ).unwrap();
        let s =  BigUint::from_str_radix(
            "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        ).unwrap();
        let n =  BigUint::from_str_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        ).unwrap();

        let px =  BigUint::from_str_radix(
            "04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574",
            16,
        ).unwrap();
        let py =  BigUint::from_str_radix(
            "82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4",
            16,
        ).unwrap();

        let point = S256Point::new(Some(S256Field::new(px)), Some(S256Field::new(py)));
        let s_inv = s.modpow( &(n.clone() - BigUint::from(2u64)), &n);

        let u = z * s_inv.clone() % n.clone();
        let v = r.clone() * s_inv % n;

        assert_eq!((S256Point::generator()*u + v * point).x.unwrap().get_num(), r);
        // (u*G + v*point).x.num 
    }
}