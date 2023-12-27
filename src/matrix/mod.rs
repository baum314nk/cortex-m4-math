use std::{
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub m: usize,
    pub n: usize,
    data: Vec<f64>,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

            let missing_str: String = std::iter::repeat(' ').take(missing_len).collect();
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

        return std::fmt::Result::Ok(());
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(
            (index.0 < self.m) && (index.1 < self.n),
            "Matrix index is out-of-bounds."
        );
        return &self.data[index.0 * self.n + index.1];
    }
}

impl IndexMut<(usize, usize)> for Matrix {
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

    // Constructors

    pub fn new<const M: usize, const N: usize>(raw_data: [[f64; N]; M]) -> Self {
        let mut data: Vec<f64> = Vec::with_capacity(M * N);
        data.extend(raw_data.iter().flat_map(|e| e.iter()));
        return Self::from(data, (M, N));
    }

    pub fn from(data: Vec<f64>, shape: (usize, usize)) -> Self {
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

    pub fn fill(value: f64, shape: (usize, usize)) -> Self {
        return Self::from(vec![value; shape.0 * shape.1], shape);
    }

    pub fn identity(size: usize) -> Self {
        let mut result = Self::zeros((size, size));

        for i in 0..size {
            result[(i, i)] = 1.0;
        }

        return result;
    }

    pub fn rotation_2d(angle: f64) -> Self {
        let (sin, cos) = angle.sin_cos();
        return Matrix::new([[cos, -sin], [sin, cos]]);
    }

    pub fn rotation_3d(yaw: f64, pitch: f64, roll: f64) -> Self {
        // Precompute trigonometric values
        let cos_yaw = yaw.cos();
        let sin_yaw = yaw.sin();
        let cos_pitch = pitch.cos();
        let sin_pitch = pitch.sin();
        let cos_roll = roll.cos();
        let sin_roll = roll.sin();

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

    // Operations

    /// Calculate the determinant of the matrix.
    pub fn determinant(&self) -> f64 {
        assert_eq!(
            self.m, self.n,
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
    fn cofactor(&self, row: usize, col: usize) -> f64 {
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
        std::mem::swap(&mut self.m, &mut self.n);
    }

    #[allow(non_snake_case)]
    /// Returns the transpose of the matrix
    pub fn T(&self) -> Matrix {
        let mut new_data = Vec::with_capacity(self.len());

        for n in 0..self.n {
            for m in 0..self.m {
                new_data.push(self[(m, n)]);
            }
        }

        return Matrix::from(new_data, (self.n, self.m));
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
}

mod operators;
