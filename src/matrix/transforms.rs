use super::Matrix;

impl Matrix {
    pub fn transform_householder(&self) -> Matrix {
        assert!(
            self.is_column_vector(),
            "Provided matrix isn't a column vector"
        );

        let rhs = &self.dyadic(&self.T()) * (2.0 / (self.T().dot(self)));

        return &Matrix::identity(self.len()) - &rhs;
    }

    // pub fn decompose_QR(&self) -> (Matrix, Matrix) {
    //     let mut Q = Matrix::identity(self.m);
    //     let R: Matrix;
    //     for k in 0..self.n {
    //         let z =
    //     }

    //     return (Q, R);
    // }
}
