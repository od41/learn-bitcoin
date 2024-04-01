use std::fmt::Display;

#[derive(Debug)]
pub struct Point {
    a: i64,
    b: i64,
    x: Option<i64>,
    y: Option<i64>,
}

impl Point {
    pub fn new(x: Option<i64>, y: Option<i64>, a: i64, b: i64) -> Point {
        if x == None || y == None {
            return Point {a, b, x: None, y: None};
        }
        let x = x.unwrap();
        let y = y.unwrap();
        if y.pow(2) != x.pow(3) + a * x + b {
            panic!("Value ({} {}) is not on the curve", x, y);
        }
        Point {a, b, x: Some(x), y: Some(y)}
    }

    pub fn eq(&self, other: &Point) -> bool {
        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = other.x.unwrap();
        let y2 = other.y.unwrap();

        x1 == x2 && y1 == y2 && self.a == other.a && self.b == other.b
    }

    pub fn ne(&self, other: &Point) -> bool {
        let x1 = self.x.unwrap();
        let y1 = self.y.unwrap();
        let x2 = other.x.unwrap();
        let y2 = other.y.unwrap();

        x1 != x2 || y1 != y2 || self.a != other.a || self.b != other.b
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

            let slope = (3*x.pow(2) + self.a) / (2 * y);

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

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn is_equals() {
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(-1), Some(-1), 5, 7);

        assert!(p2.eq(&p3));
    }

    #[test]
    fn is_not_equal() {
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(18), Some(-77), 5, 7);
        let infinity = Point::new(None, None, 5, 7);

        println!("ECP at infinity {:?}", infinity);

        assert!(p2.ne(&p3));
    }

    #[test]
    fn add_to_identity() {
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let infinity = Point::new(None, None, 5, 7);

        assert!(p2.add(&infinity).eq(&p2));
        assert!(infinity.add(&p2).eq(&p2));
    }

    #[test]
    fn add_to_point() {
        // let p2 = Point::new(Some(2), Some(5), 5, 7);
        // let p3 = Point::new(Some(-1), Some(-1), 5, 7);
        // let sum = Point::new(Some(3), Some(-7), 5, 7);

        // assert!(p2.add(&p3).eq(&sum));

        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(-1), Some(-1), 5, 7);
        let sum = Point::new(Some(18), Some(77), 5, 7);

        println!("sum is {:?}", p2.add(&p3));
        assert!(p2.add(&p3).eq(&sum));
    }

}