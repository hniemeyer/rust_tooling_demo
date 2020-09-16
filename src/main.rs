mod geometry;

fn main() {
    let rect = geometry::Rectangle::new(3.2, 4.5);
    println!("{}", rect.area());
    let serialized = serde_json::to_string(&rect).unwrap();
    println!("serialized = {}", serialized);

}
