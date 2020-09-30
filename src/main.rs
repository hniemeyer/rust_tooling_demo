mod geometry;

fn main() {
    let mut rect = geometry::Rectangle::new(3.2, 4.5, 21.2, 33.2);
    rect.move_direction(1.2, 3.2);
    println!("{}", rect.area());
    println!("{}", rect.circumference());
    let serialized = serde_json::to_string(&rect).unwrap();
    println!("serialized = {}", serialized);
}
