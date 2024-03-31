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
            Point{
                x: other.x,
                y: other.y,
                a: other.a,
                b: other.b
            }
        } else if other.x == None || other.y == None {
            Point{
                x: self.x,
                y: self.y,
                a: self.a,
                b: self.b
            }
        } else  {
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
        // let p1 = Point::new(2, 4, 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = Point::new(Some(18), Some(-77), 5, 7);
        // let p4 = Point::new(5, 7, 5, 7);

        // assert!(!p1.eq(p2));
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

}