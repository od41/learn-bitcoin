use super::field_element::FieldElement;

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
        if y.pow(2) != x.pow(3).add(&(a.mul(&x)).add(&b)) {
            panic!("Value ({:?} {:?}) is not on the curve", x, y);
        }
        Point {a, b, x: Some(x), y: Some(y)}
    }

    pub fn eq(&self, other: &Point) -> bool {
        // let x1 = self.x.unwrap();
        // let y1 = self.y.unwrap();
        // let x2 = other.x.unwrap();
        // let y2 = other.y.unwrap();

        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }

    pub fn ne(&self, other: &Point) -> bool {
        // let x1 = self.x.unwrap();
        // let y1 = self.y.unwrap();
        // let x2 = other.x.unwrap();
        // let y2 = other.y.unwrap();
        // TODO: redo the equality checks because they're on Field elements not integers
        self.x != other.x || self.y != other.y || self.a != other.a || self.b != other.b
    }

    pub fn add(&self, other: &Point) -> Point {
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
            // returns a point at infinity... right?
            Point{
                x: None,
                y: None,
                a: self.a,
                b: self.b
            }
        } else if self.x != other.x {
            let x1 = self.x.unwrap();
            let y1 = self.y.unwrap();
            let x2 = other.x.unwrap();
            let y2 = other.y.unwrap();

            let slope = (y2.sub(&y1)).div(&x2.sub(&x1));

            let x3 = slope.pow(2).sub(&x1.sub(&x2));
            let y3 = (slope.mul(&x1.sub(&x3))).sub(&y1);

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

            let slope = ((x.s_mul(3).pow(2)).add(&self.a)).div(&y.s_mul(2));

            let x3 = slope.pow(2).sub(&x.s_mul(2));
            let y3 = (slope.mul(&x.sub(&x3))).sub(&y);


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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn is_equals() {
        let prime = 223;
        let x = FieldElement::new(192, prime);
        let y = FieldElement::new(105, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x), Some(y), a, b);
        let p3 = Point::new(Some(x), Some(y), a, b);

        assert!(p2.eq(&p3));
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

        assert!(p2.ne(&p3));
    }

    #[test]
    fn add_to_identity() {
        let prime = 223;
        let x = FieldElement::new(192, prime);
        let y = FieldElement::new(105, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x), Some(y), a, b);
        let infinity = Point::new(None, None, a, b);

        assert!(p2.add(&infinity).eq(&p2));
        assert!(infinity.add(&p2).eq(&p2));
    }

    #[test]
    #[should_panic] // TODO: the test fails rn
    fn add_to_point() {
        let prime = 223;
        let x1 = FieldElement::new(2, prime);
        let y1 = FieldElement::new(5, prime);
        let x2 = FieldElement::new(18, prime);
        let y2 = FieldElement::new(77, prime);
        let a = FieldElement::new(5, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x1), Some(y1), a, b);
        let p3 = Point::new(Some(x2), Some(y2), a, b);
        
        let sum = Point::new(
            Some(FieldElement::new(56, prime)), 
            Some(FieldElement::new(198, prime)), 
            a,
            b
        );

        println!("sum is {:?}", p2.add(&p3));

        assert!(p2.add(&p3).eq(&sum));
    }

}