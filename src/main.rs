mod matrix;

use matrix::Matrix;

fn main() {
    let mut mat1 = Matrix::new([[4.0, 0.0, 4.0], [3.0, -4.0, 3.0], [4.0, -2.0, 1.0]]);

    println!("{}", mat1.determinant())
}
