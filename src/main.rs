use ndarray::prelude::*;

fn change_elem(x: &mut Array2<f64>) {

    x[[0,0]] += 1.0;

}

fn main() {
    let mut a = array![
                [1.,2.,3.], 
                [4.,5.,6.],
            ]; 
    change_elem(&mut a);
    println!("{:?}", a);
}
