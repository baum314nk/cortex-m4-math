use super::Matrix;
use std::ops::{Add, Mul, MulAssign, Neg, Sub};

impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut result_data = Vec::with_capacity(self.m * self.n);

        for e in &self.data {
            result_data.push(e * rhs);
        }

        return Matrix::from(result_data, self.shape());
    }
}

impl MulAssign<f64> for Matrix {
    fn mul_assign(&mut self, rhs: f64) {
        for e in &mut self.data {
            *e *= rhs;
        }
    }
}

impl Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &Matrix) -> Self::Output {
        assert!(
            self.n == rhs.m,
            "Can't multiply {}x{} matrix with {}x{} matrix.",
            self.m,
            self.n,
            rhs.m,
            rhs.n,
        );

        let mut result_data: Vec<f64> = Vec::with_capacity(self.m * rhs.n);
        for m in 0..self.m {
            for o in 0..rhs.n {
                let value = (0..self.n).map(|n| self[(m, n)] * rhs[(n, o)]).sum();
                result_data.push(value);
            }
        }

        return Matrix::from(result_data, (self.m, rhs.n));
    }
}

impl MulAssign<&Matrix> for Matrix {
    fn mul_assign(&mut self, rhs: &Matrix) {
        assert!(
            self.n == rhs.m,
            "Can't multiply {}x{} matrix with {}x{} matrix.",
            self.m,
            self.n,
            rhs.m,
            rhs.n,
        );

        let mut result_data: Vec<f64> = Vec::with_capacity(self.m * rhs.n);
        for m in 0..self.m {
            for o in 0..rhs.n {
                let value = (0..self.n).map(|n| self[(m, n)] * rhs[(n, o)]).sum();
                result_data.push(value);
            }
        }

        self.data = result_data;
        std::mem::swap(&mut self.m, &mut self.n);
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        assert!(
            self.shape() == rhs.shape(),
            "Can't add {}x{} matrix to {}x{} matrix.",
            self.m,
            self.n,
            rhs.m,
            rhs.n,
        );

        let mut result_data: Vec<f64> = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            result_data.push(self.data[i] + rhs.data[i]);
        }

        return Matrix::from(result_data, self.shape());
    }
}

impl Sub<&Matrix> for &Matrix {
    type Output = Matrix;

    fn sub(self, rhs: &Matrix) -> Self::Output {
        assert!(
            self.shape() == rhs.shape(),
            "Can't subtract {}x{} matrix from {}x{} matrix.",
            self.m,
            self.n,
            rhs.m,
            rhs.n,
        );

        let mut result_data: Vec<f64> = Vec::with_capacity(self.len());

        for i in 0..self.len() {
            result_data.push(self.data[i] - rhs.data[i]);
        }

        return Matrix::from(result_data, self.shape());
    }
}

impl Neg for &Matrix {
    type Output = Matrix;

    fn neg(self) -> Self::Output {
        let mut result_data = Vec::with_capacity(self.m * self.n);

        for e in &self.data {
            result_data.push(-e);
        }

        return Matrix::from(result_data, self.shape());
    }
}
