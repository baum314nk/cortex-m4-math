mod matrix;

use matrix::Matrix;

fn main() {
    let v = Matrix::new([[1.0], [3.0], [2.0]]);
    println!("{}\n", v);
}
