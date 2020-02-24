pub mod matrix {
    use crate::matrix::error::OutOfBoundsError;

    pub mod error {
        use std::fmt;
        use std::error;

        pub struct OutOfBoundsError;
        impl fmt::Display for OutOfBoundsError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Item specified is outside of matrix!")
            }
        }

        impl error::Error for OutOfBoundsError {
            fn source(&self) -> Option<&(dyn error::Error + 'static)> {
                None
            }
        }
    }

    fn cap(a: usize, b: usize) -> Vec<i32> {
        Vec::with_capacity(a * b - 1)
    }

    pub struct Matrix {
        rows: usize,
        cols: usize,
        entries: Vec<i32>
    }

    impl Matrix {

        pub fn from(rows: usize, cols: usize, entries: Vec<i32>) -> Matrix {
            Matrix {
                rows,
                cols,
                entries
            }
        }

        pub fn new(rows: usize, cols: usize) -> Matrix {
            Matrix::from(rows, cols, cap(rows, cols))
        }

        pub fn identity(size: usize) -> Matrix {
            let mut entries = cap(size, size);
            for i in 0..size {
                entries[i * size - 1] = 1;
            }
            Matrix::from(size, size, entries)
        }

//        pub fn transpose(&self) -> Matrix {
//            let num_entries = self.rows * self.cols - 1;
//            let mut entries = Vec::with_capacity(num_entries);
//            for i in 1..num_entries {
//                entries[i] = self.entries[i + self.rows % num_entries]
//            }
//            Matrix::from(cols, rows, entries)
//        }

        pub fn get(&self, row: usize, col: usize) -> i32 {
            self.entries[((row - 1) * self.cols) + col - 1]
        }

//        pub fn row(&self, i: usize) -> Result<[i32; usize], OutOfBoundsError> {
//            if i > 0 && i <= self.rows {
//                Ok(self.entries[i-1])
//            } else {
//                Err(OutOfBoundsError)
//            }
//        }

//        pub fn col(&self, i: usize) -> Result<[i32; usize], OutOfBoundsError> {
//            if i > 0 && i <= self.cols {
//                Ok(self.transpose().row(i)?) //unnecessarily expensive?
//            } else {
//                Err(OutOfBoundsError)
//            }
//        }
    }
}


#[cfg(test)]
mod tests {
    use crate::matrix::*;
    #[test]
    fn from_array() {
        let mini_mat = [1,2,3,4];
        let my_matrix = Matrix::from(2, 2, mini_mat);
        assert_eq!(my_matrix.get(1,1), 1);
        assert_eq!(my_matrix.get(1,2), 2);
        assert_eq!(my_matrix.get(2,1), 3);
        assert_eq!(my_matrix.get(2,2), 4);
    }
}
