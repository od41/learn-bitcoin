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
        if y.pow(2) != x.pow(3) + (a * x) + b {
            panic!("Value ({:?} {:?}) is not on the curve", x, y);
        }
        Point {a, b, x: Some(x), y: Some(y)}
    }

    pub fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.a == other.a && self.b == other.b
    }

    pub fn ne(&self, other: &Point) -> bool {
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

            let slope = (y2 - y1) / (x2 - x1);

            let x3 = slope.pow(2) - x1 - x2;
            let y3 = (slope * x1) - x3 - y1;

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

            // TODO

            // let slope = ((3 * x).pow(2)) + self.a / (2 * y);

            // let x3 = slope.pow(2) - (2 * x);
            // let y3 = (slope * (x - x3) - y);


            // Point{
            //     x: Some(x3),
            //     y: Some(y3),
            //     a: self.a,
            //     b: self.b
            // }

            Point{
                x: self.x,
                y: self.y,
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
    // #[should_panic] // TODO: the test fails rn
    fn add_to_point() {
        let prime = 223;
        let x1 = FieldElement::new(192, prime);
        let y1 = FieldElement::new(105, prime);
        let x2 = FieldElement::new(17, prime);
        let y2 = FieldElement::new(56, prime);
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);

        let p2 = Point::new(Some(x1), Some(y1), a, b);
        let p3 = Point::new(Some(x2), Some(y2), a, b);
        
        let sum = Point::new(
            Some(FieldElement::new(170, prime)), 
            Some(FieldElement::new(142, prime)), 
            a,
            b
        );

        println!("sum is {:?}", p2.add(&p3));

        assert!(p2.add(&p3).eq(&sum));
    }

    #[test]
    fn test_on_curve() {
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
    fn test_on_curve_invalid() {
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

}