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

    pub struct Matrix {
        rows: usize,
        cols: usize,
        entries: [[i32; usize]; usize]
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Matrix {
            Matrix {
                rows,
                cols,
                entries: [[0; rows]; cols]
            }
        }

        pub fn identity(size: usize) -> Matrix {
            let mut entries = [[0; size]; size];
            for i in 0..size {
                entries[i][i] = 1;
            }
            Matrix {
                rows: size,
                cols: size,
                entries
            }
        }

        pub fn transpose(&self) -> Matrix {
            let mut entries = [[0; self.cols]; self.rows];
            for (row, col) in (self.rows, self.cols) {
                entries[col][row] = self.entries[row][col];
            }
            Matrix {
                rows: self.cols,
                cols: self.rows,
                entries
            }
        }

        pub fn row(&self, i: usize) -> Result<[i32; usize], OutOfBoundsError> {
            if i > 0 && i <= self.rows {
                Ok(self.entries[i])
            } else {
                Err(OutOfBoundsError)
            }
        }

        pub fn col(&self, i: usize) -> Result<[i32; usize], OutOfBoundsError> {
            if i > 0 && i <= self.cols {
                Ok(self.transpose().row(i)?) //unnecessarily expensive?
            } else {
                Err(OutOfBoundsError)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
