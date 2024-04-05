#![allow(unused)]
use super::field_element::FieldElement;
use std::{fmt, ops::{Add, Mul}};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    a: FieldElement,
    b: FieldElement,
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl Point {
    pub fn new(x: Option<FieldElement>, y: Option<FieldElement>, a: FieldElement, b: FieldElement) -> Point {
        if x == None || y == None {
            return Point {a, b, x: None, y: None};
        }
        let x = x.unwrap();
        let y = y.unwrap();
        if y.pow(2) != x.pow(3) + (a * x) + b {
            panic!("Value ({:?} {:?}) is not on the curve", x, y);
        }
        Point {a, b, x: Some(x), y: Some(y)}
    }    
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }

    fn ne(&self, other: &Point) -> bool {
        self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b
    }
}
impl Eq for Point {}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points {:?}, {:?} are not on the same curve", self, other);
        }

        if self.x == None || self.y == None {
            // return other
            Point{
                x: other.x,
                y: other.y,
                a: other.a,
                b: other.b
            }
        } else if other.x == None || other.y == None {
            // return self
            Point{
                x: self.x,
                y: self.y,
                a: self.a,
                b: self.b
            }
        } else if self.x == other.x && self.y !=  other.y {
            // returns a point at infinity...it's a vertical line
            Point{
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

            let slope = (y2 - y1) / (x2 - x1);

            let x3 = slope.pow(2) - x1 - x2;
            let y3 = (slope * (x1 - x3)) - y1;

            Point{
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b
            }
        } else if self.x == other.x && self.y == other.y {
            // they are at the same point and you calculate 
            // the slope of the tangent to that point
            let x = self.x.unwrap();
            let y = self.y.unwrap();

            let slope = ((3 * x.pow(2)) + self.a) / (2 * y);

            let x3 = slope.pow(2) - (2 * x);
            let y3 = (slope * (x - x3)) - y;


            Point{
                x: Some(x3),
                y: Some(y3),
                a: self.a,
                b: self.b
            }
        } else {
            Point{
                x: self.x,
                y: self.y,
                a: self.a,
                b: self.b
            }
        }
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, coefficient: usize) -> Self {
        let mut product = Point {x: None, y: None, a: self.a, b: self.b};
        
        for _ in 0..coefficient {
            product = product + self;
        }
        
        product
    }
}

impl Mul<Point> for usize {
    type Output = Point;

    fn mul(self, other: Point) -> Self::Output {
        let mut product = Point {x: None, y: None, a: other.a, b: other.b};
        
        for _ in 0..self {
            product = product + other;
        }
        
        product
    }
}


#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn is_valid_curves() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let valid_points = vec![(192, 105), (17, 56), (1, 193)];

        for (x_raw, y_raw) in valid_points {
            let x = FieldElement::new(x_raw, prime);
            let y = FieldElement::new(y_raw, prime);
            Point::new(Some(x), Some(y), a, b);
        } 
    }

    #[test]
    #[should_panic]
    fn is_invalid_curves() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let invalid_points = vec![(200, 119), (42, 99)];

        for (x_raw, y_raw) in invalid_points {
            let x = FieldElement::new(x_raw, prime);
            let y = FieldElement::new(y_raw, prime);
            Point::new(Some(x), Some(y), a, b);
        }
    }

    #[test]
    fn is_equals() {
        let prime = 223;
        let x = FieldElement::new(192, prime);
        let y = FieldElement::new(105, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x), Some(y), a, b);
        let p3 = Point::new(Some(x), Some(y), a, b);

        assert!(p2 == p3);
    }

    #[test]
    fn is_not_equal() {
        let prime = 223;
        let x1 = FieldElement::new(2, prime);
        let y1 = FieldElement::new(5, prime);
        let x2 = FieldElement::new(18, prime);
        let y2 = FieldElement::new(77, prime);
        let a = FieldElement::new(5, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x1), Some(y1), a, b);
        let p3 = Point::new(Some(x2), Some(y2), a, b);

        assert!(p2 != p3);
    }

    #[test]
    fn add_to_identity_point() {
        let prime = 223;
        let x = FieldElement::new(192, prime);
        let y = FieldElement::new(105, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x), Some(y), a, b);
        let infinity = Point::new(None, None, a, b);

        assert!(p2 + infinity == p2);
        assert!(infinity + p2 == p2);
    }
    
    #[test]
    #[should_panic]
    fn add_to_same_x_point() {
        // x1 == x2 but y1 != y2
        let prime = 223;
        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let x2 = FieldElement::new(192, prime);
        let y2 = FieldElement::new(0, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x1), Some(y1), a, b);
        let p3 = Point::new(Some(x2), Some(y2), a, b);
        
        let sum = Point::new(
            Some(FieldElement::new(160, prime)), 
            Some(FieldElement::new(101, prime)), 
            a,
            b
        );

        assert!(p2 + p3 == sum);
    }

    #[test]
    fn add_to_tangential_point() {
        // the point is at a tangent and P1 == P2
        let prime = 223;
        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p1 = Point::new(Some(x1), Some(y1), a, b);
        
        let sum = Point::new(
            Some(FieldElement::new(49, prime)), 
            Some(FieldElement::new(71, prime)), 
            a,
            b
        );

        assert!(p1 + p1 == sum);
    }

    #[test]
    fn add_to_different_points() {
        // x1 != x2 and y1 != y2
        let prime = 223;
        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let x2 = FieldElement::new(206, prime);
        let y2 = FieldElement::new(0, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x1), Some(y1), a, b);
        let p3 = Point::new(Some(x2), Some(y2), a, b);
        
        let sum = Point::new(
            Some(FieldElement::new(160, prime)), 
            Some(FieldElement::new(101, prime)), 
            a,
            b
        );

        assert!(p2 + p3 == sum);
    }


    #[test]
    fn scalar_mul_works() {
        let prime = 223;
        let x = FieldElement::new(15, prime);
        let y = FieldElement::new(86, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p = Point::new(Some(x), Some(y), a, b);
        let infinity = Point::new(None, None, a, b);
        let product = 7 * p;
        assert!(product == infinity);
    }
}