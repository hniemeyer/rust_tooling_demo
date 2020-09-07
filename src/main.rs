use ndarray::prelude::*;
mod geometry;

fn change_elem(x: &mut Array2<f64>) {

    x[[0,0]] += 1.0;

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let mut a = array![
                [1.,2.,3.], 
                [4.,5.,6.],
            ]; 
            change_elem(&mut a);

        assert_eq!(a[[0,0]],2.0);
    }
}

fn main() {
    let mut a = array![
                [1.,2.,3.], 
                [4.,5.,6.],
            ]; 
    let rect = geometry::Rectangle::new(3.2, 4.5);
    change_elem(&mut a);
    println!("{:?}", a);
    println!("{}", rect.area());
}
