use serde::{Serialize, Deserialize};
/// A geomtric rectangle
#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    /// length of the rectangle
    length: f64,
    /// width of the rectangle
    width: f64
}

impl Rectangle {
    /// create a new rectangle.
    pub fn new(l: f64, w: f64) -> Self {
        Rectangle {length: l, width: w}
    }

    /// calculate the area of the rectangle
    pub fn area(&self) -> f64 {
        self.length * self.width
    }

    pub fn circumference(&self) -> f64 {
        2.0 * self.length + 2.0 * self.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area() {
        let my_rect = Rectangle::new(2.0, 2.0);
        assert_eq!(my_rect.area(), 4.0);
    }

    #[test]
   fn circumference() {
        let my_rect = Rectangle::new(2.0, 3.0);
        assert_eq!(my_rect.circumference(), 10.0)
    }
}