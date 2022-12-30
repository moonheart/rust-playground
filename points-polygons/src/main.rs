use std::f64::consts::PI;
use std::ops::Add;
use std::slice::Iter;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn magnitude(&self) -> f64 {
        f64::sqrt((self.x.pow(2) + self.y.pow(2)).into())
    }

    fn dist(&self, p: Point) -> f64 {
        f64::sqrt(((self.x - p.x).pow(2) + (self.y - p.y).pow(2)).into())
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Polygon { points: vec![] }
    }

    fn add_point(&mut self, p: Point) {
        self.points.push(p)
    }

    fn left_most_point(self) -> Option<Point> {
        let mut left: Option<&Point> = None;
        for p in self.points.iter() {
            match left {
                Some(l) if l.x < p.x => (),
                _ => left = Some(p),
            }
        }
        return Some(*left.unwrap());
    }

    fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    fn circumference(&self) -> f64 {
        let mut res = 0.0;
        let len = self.points.len();
        if len < 3 {
            return res;
        }
        for i in 0..len - 1 {
            res += self.points[i].dist(self.points[i + 1])
        }
        res += self.points[0].dist(self.points[len - 1]);
        return res;
    }
}

pub struct Circle {
    point: Point,
    radius: i32,
}

impl Circle {
    fn new(p: Point, r: i32) -> Self {
        Circle {
            point: p,
            radius: r,
        }
    }
    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius as f64
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn circumference(s: &Shape) -> f64 {
        match s {
            Shape::Circle(c) => c.circumference(),
            Shape::Polygon(p) => p.circumference(),
        }
    }
}

impl From<Polygon> for Shape {
    fn from(p: Polygon) -> Self {
        Shape::Polygon(p)
    }
}

impl From<Circle> for Shape {
    fn from(p: Circle) -> Self {
        Shape::Circle(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}
