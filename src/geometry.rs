pub struct Rectangle {
    length: f64,
    width: f64
}

impl Rectangle {
    pub fn new(l: f64, w: f64) -> Self {
        Rectangle {length: l, width: w}
    }

    pub fn area(&self) -> f64 {
        self.length * self.width
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
}