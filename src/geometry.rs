use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(xx: f64, yy: f64) -> Self {
        Point { x: xx, y: yy }
    }

    fn move_direction(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
}

/// A geomtric rectangle
#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    /// length of the rectangle
    length: f64,
    /// width of the rectangle
    width: f64,
    /// center of the rectangle
    center: Point,
}

impl Rectangle {
    /// create a new rectangle.
    pub fn new(l: f64, w: f64, x: f64, y: f64) -> Self {
        let p = Point::new(x, y);
        Rectangle {
            length: l,
            width: w,
            center: p,
        }
    }

    /// calculate the area of the rectangle
    pub fn area(&self) -> f64 {
        self.length * self.width
    }

    pub fn circumference(&self) -> f64 {
        2.0 * self.length + 2.0 * self.width
    }

    pub fn move_direction(&mut self, dx: f64, dy: f64) {
        self.center.move_direction(dx, dy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let my_rect = Rectangle::new(2.0, 2.0, 0.0, 0.0);
        assert_eq!(my_rect.area(), 4.0);
    }

    #[test]
    fn circumference() {
        let my_rect = Rectangle::new(2.0, 3.0, 0.0, 0.0);
        assert_eq!(my_rect.circumference(), 10.0)
    }

    #[test]
    fn move_point() {
        let mut my_point = Point::new(1.0, 1.0);
        my_point.move_direction(-1.0, -1.0);
        assert_eq!(my_point.x, 0.0);
        assert_eq!(my_point.y, 0.0);
    }

    #[test]
    fn move_rectangle() {
        let mut my_rect = Rectangle::new(1.0, 1.0, 1.0, 1.0);
        my_rect.move_direction(-1.0, -1.0);
        assert_eq!(my_rect.center.x, 0.0);
        assert_eq!(my_rect.center.y, 0.0);
    }
}
