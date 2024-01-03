use alloc::string::String;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::{fmt, iter, ops};
use libm::{cosf, sincosf, sinf};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub m: usize,
    pub n: usize,
    data: Vec<f32>,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert all numbers to strings
        let mut str_data = Vec::with_capacity(self.len());
        let mut max_len: usize = 0;

        for m in 0..self.m {
            for n in 0..self.n {
                let str_date = format!("{:.3}", self[(m, n)]);
                let str_len = str_date.len();
                if str_len > max_len {
                    max_len = str_len;
                }

                str_data.push(str_date);
            }
        }

        // Prepend spaces to numbers to fill up missing space
        for str_date in &mut str_data {
            let missing_len = max_len - str_date.len();
            if missing_len == 0 {
                continue;
            }

            let missing_str: String = iter::repeat(' ').take(missing_len).collect();
            str_date.insert_str(0, &missing_str);
        }

        macro_rules! write_row {
            ($m:expr) => {
                write!(f, "| {}", str_data[$m * self.n])?;
                for n in 1..self.n {
                    write!(f, "  {}", str_data[$m * self.n + n])?;
                }
                write!(f, " |")?;
            };
        }

        write_row!(0);
        for m in 1..self.m {
            write!(f, "\n")?;
            write_row!(m);
        }

        return Result::Ok(());
    }
}

impl ops::Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(
            (index.0 < self.m) && (index.1 < self.n),
            "Matrix index is out-of-bounds."
        );
        return &self.data[index.0 * self.n + index.1];
    }
}

impl ops::IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(
            (index.0 < self.m) && (index.1 < self.n),
            "Matrix index is out-of-bounds."
        );
        return &mut self.data[index.0 * self.n + index.1];
    }
}

impl Matrix {
    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn shape(&self) -> (usize, usize) {
        return (self.m, self.n);
    }

    pub fn is_quadratic(&self) -> bool {
        return self.m == self.n;
    }

    pub fn is_row_vector(&self) -> bool {
        return self.m == 1;
    }

    pub fn is_column_vector(&self) -> bool {
        return self.n == 1;
    }

    pub fn is_vector(&self) -> bool {
        return self.is_column_vector() || self.is_row_vector();
    }

    // Constructors

    pub fn new<const M: usize, const N: usize>(raw_data: [[f32; N]; M]) -> Self {
        let mut data: Vec<f32> = Vec::with_capacity(M * N);
        data.extend(raw_data.iter().flat_map(|e| e.iter()));
        return Self::from(data, (M, N));
    }

    pub fn from(data: Vec<f32>, shape: (usize, usize)) -> Self {
        assert!(
            data.len() == shape.0 * shape.1,
            "Length of data {} doesn't match {}x{} shape of the matrix.",
            data.len(),
            shape.0,
            shape.1
        );
        return Self {
            m: shape.0,
            n: shape.1,
            data,
        };
    }

    pub fn zeros(shape: (usize, usize)) -> Self {
        return Self::fill(0.0, shape);
    }

    pub fn fill(value: f32, shape: (usize, usize)) -> Self {
        return Self::from(vec![value; shape.0 * shape.1], shape);
    }

    pub fn identity(size: usize) -> Self {
        let mut result = Self::zeros((size, size));

        for i in 0..size {
            result[(i, i)] = 1.0;
        }

        return result;
    }

    pub fn rotation_2d(angle: f32) -> Self {
        let (sin, cos) = sincosf(angle);
        return Matrix::new([[cos, -sin], [sin, cos]]);
    }

    pub fn rotation_3d(yaw: f32, pitch: f32, roll: f32) -> Self {
        // Precompute trigonometric values
        let cos_yaw = cosf(yaw);
        let sin_yaw = sinf(yaw);
        let cos_pitch = cosf(pitch);
        let sin_pitch = sinf(pitch);
        let cos_roll = cosf(roll);
        let sin_roll = sinf(roll);

        // Directly construct the rotation matrix
        return Matrix::new([
            [
                cos_yaw * cos_pitch,
                cos_yaw * sin_pitch * sin_roll - sin_yaw * cos_roll,
                cos_yaw * sin_pitch * cos_roll + sin_yaw * sin_roll,
            ],
            [
                sin_yaw * cos_pitch,
                sin_yaw * sin_pitch * sin_roll + cos_yaw * cos_roll,
                sin_yaw * sin_pitch * cos_roll - cos_yaw * sin_roll,
            ],
            [-sin_pitch, cos_pitch * sin_roll, cos_pitch * cos_roll],
        ]);
    }

    // Data access

    pub fn get_rows(&self, row_slice: &[usize]) -> Self {
        let mut data = Vec::with_capacity(self.n * row_slice.len());

        for m in row_slice {
            for n in 0..self.n {
                data.push(self[(*m, n)])
            }
        }

        return Self::from(data, (row_slice.len(), self.n));
    }

    pub fn get_column(&self, column_idx: usize) -> Self {
        let mut data = Vec::with_capacity(self.m);

        for m in 0..self.m {
            data.push(self[(m, column_idx)])
        }

        return Self::from(data, (self.m, 1));
    }

    // Operations

    /// Calculate the determinant of the matrix.
    pub fn determinant(&self) -> f32 {
        assert!(
            self.is_quadratic(),
            "Matrix must be square for determinant calculation"
        );

        if self.m == 1 {
            // For a 1x1 matrix, the determinant is the single element
            return self[(0, 0)];
        } else {
            let mut det = 0.0;

            for n in 0..self.n {
                det += self[(0, n)] * self.cofactor(0, n);
            }

            return det;
        }
    }

    /// Helper function to compute the cofactor of a matrix element
    fn cofactor(&self, row: usize, col: usize) -> f32 {
        let minor_matrix = self.minor_matrix(row, col);
        let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };

        return sign * minor_matrix.determinant();
    }

    /// Helper function to obtain the minor matrix by excluding a row and column
    fn minor_matrix(&self, row_exclude: usize, col_exclude: usize) -> Matrix {
        assert!(
            row_exclude < self.m && col_exclude < self.n,
            "Row or column index out of bounds."
        );

        let mut minor_data = Vec::with_capacity((self.m - 1) * (self.n - 1));

        for m in 0..self.m {
            for n in 0..self.n {
                if m != row_exclude && n != col_exclude {
                    minor_data.push(self[(m, n)]);
                }
            }
        }

        return Matrix::from(minor_data, (self.m - 1, self.n - 1));
    }

    #[allow(non_snake_case)]
    /// Transposes the matrix inplace
    pub fn T_ip(&mut self) {
        let mut new_data = Vec::with_capacity(self.len());

        for n in 0..self.n {
            for m in 0..self.m {
                new_data.push(self[(m, n)]);
            }
        }

        self.data = new_data;
        core::mem::swap(&mut self.m, &mut self.n);
    }

    #[allow(non_snake_case)]
    /// Returns the transpose of the matrix
    pub fn T(&self) -> Matrix {
        let mut result = self.clone();
        result.T_ip();

        return result;
    }

    pub fn dot(&self, rhs: &Matrix) -> f32 {
        assert!(
            self.is_row_vector() && rhs.is_column_vector(),
            "Provided matrices aren't vectors of the correct form."
        );
        assert!(self.len() == rhs.len(), "Vectors aren't of same size");

        return self
            .data
            .iter()
            .zip(rhs.data.iter())
            .map(|(v1, v2)| v1 * v2)
            .sum();
    }

    pub fn dyadic(&self, rhs: &Matrix) -> Matrix {
        assert!(
            self.is_column_vector() && rhs.is_row_vector(),
            "Provided matrices aren't vectors of the correct form"
        );

        let mut result_data = Vec::with_capacity(self.m * rhs.n);

        for m in 0..self.m {
            for n in 0..rhs.n {
                result_data.push(self.data[m] * rhs.data[n])
            }
        }

        return Matrix::from(result_data, (self.m, rhs.n));
    }

    pub fn cross(&self, rhs: &Matrix) -> Matrix {
        assert!(
            self.is_column_vector() && self.len() == 3 && rhs.is_column_vector() && rhs.len() == 3,
            "Provided matrices aren't column vectors of length 3"
        );

        let l = &self.data;
        let r = &rhs.data;
        return Matrix::new([
            [l[1] * r[2] - l[2] * r[1]],
            [l[2] * r[0] - l[0] * r[2]],
            [l[0] * r[1] - l[1] * r[0]],
        ]);
    }

    pub fn pow(&self, exponent: u64) -> Matrix {
        assert!(
            self.is_quadratic(),
            "Can't calculate the exponentiation as the matrix isn't quadratic"
        );

        let mut result = self.clone();
        result.pow_ip(exponent);
        return result;
    }

    pub fn pow_ip(&mut self, exponent: u64) {
        assert!(
            self.is_quadratic(),
            "Can't calculate the exponentiation as the matrix isn't quadratic"
        );

        if exponent == 0 {
            // Set the matrix to the identity matrix if exponent is 0
            *self = Matrix::identity(self.m);
            return;
        } else if exponent == 1 {
            return;
        }

        if exponent == 2 {
            *self *= &self.clone();
        } else if exponent % 2 == 0 {
            self.pow_ip(exponent / 2);
            *self *= &self.clone();
        } else {
            let orig = self.clone();
            self.pow_ip(exponent / 2);
            *self *= &self.clone();
            *self *= &orig;
        }
    }

    pub fn norm_l2(&self) -> f32 {
        let result: f32 = self.data.iter().map(|e| e * e).sum();
        return result / (self.len() as f32);
    }
}

mod operators;
mod transforms;
