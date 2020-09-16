mod geometry;

fn main() {
    let rect = geometry::Rectangle::new(3.2, 4.5);
    println!("{}", rect.area());
}
