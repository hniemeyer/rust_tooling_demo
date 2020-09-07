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

