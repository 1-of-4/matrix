pub mod error;

#[macro_use]
pub mod matrix {
    use crate::error::*;
    type Result<T> = std::result::Result<T, MatrixError>;

    /// Similar to stdlib's `vec!`, creates a Matrix object without necessarily needing needing any other data structures or complex syntax.
    ///
    /// Syntax is `mat!(number_of_rows; number_of_columns; [list of values]` if you're going straight from raw data.
    /// - The explicit list of values can be signed or unsigned integers or floating point values.
    /// - You should *always* bind to a mutable variable, since many functions take a `&mut Matrix`.
    /// - As with most macro invocations, spacing is optional. However the `;` and `[`+`]` are not.
    ///
    /// If you want to build from a preexisting `Vec<f64>`, use [Matrix::create()] instead. If you have a `Vec` of some other numeric type, `map` its contents to f64 first.
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[macro_use]
    /// # use calc::{matrix::Matrix, mat};
    /// # fn main() {
    ///
    /// // constructs a 2x2 matrix with content:
    /// // (row, col) : data
    /// // (1, 1) : 1
    /// // (1, 2) : 2
    /// // (2, 1) : 3
    /// // (2, 2) : 4
    /// let mut from_macro = mat!(2; 2; [1, 2, 3, 4]);
    ///
    /// assert_eq!(from_macro.entry(1, 1), 1.0);
    /// assert_eq!(from_macro.entry(1, 2), 2.0);
    /// assert_eq!(from_macro.entry(2, 1), 3.0);
    /// assert_eq!(from_macro.entry(2, 2), 4.0);
    /// # }
    /// ```
    #[macro_export]
    macro_rules! mat {
        (
            $(
                $r:expr; $c:expr; [$($e:expr),+]
            )?
        ) => {{
            let vec = vec![$($($e as f64),+),*];
            Matrix::create($($r)?, $($c)?, vec)
        }}
    }

    /// Row operation to swap two rows within a matrix.
    /// Fields `r1` and `r2` are the rows you want to swap.
    pub struct Swap {
        pub r1: usize,
        pub r2: usize
    }

    /// Row operation to sum two rows, replacing the first row with the sum.
    /// Field `r1` is the row that will be replaced.
    pub struct Sum {
        pub r1: usize,
        pub r2: usize
    }

    /// Row operation to multiply a row by a coefficient.
    /// `r` is the row number, `co` is the coefficient.
    pub struct Multiply {
        pub r: usize,
        pub co: f64
    }

    pub enum RowOp {
        Swap(Swap),
        Sum(Sum),
        Multiply(Multiply)
    }

    /// Trait for all row operations, only has one function: `operate()`.
    pub trait RowOperation {
        fn operate(&self, m: &mut Matrix) -> Result<()>;
    }

    impl RowOperation for RowOp {
        fn operate(&self, m: &mut Matrix) -> Result<()>{
            match self {
                RowOp::Swap(inner) => inner.operate(m),
                RowOp::Sum(inner) => inner.operate(m),
                RowOp::Multiply(inner) => inner.operate(m)
            }
        }
    }

    impl RowOperation for Swap {
        fn operate(&self, m: &mut Matrix) -> Result<()> {
            check_index(self.r1, m.r)?;
            check_index(self.r2, m.r)?;
            for col in 1..=m.c {
                let v1 = m.entry(self.r1, col);
                let v2 = m.entry(self.r2, col);
                m.update(self.r1, col, v2);
                m.update(self.r2, col, v1);
            }
            Ok(())
        }
    }

    impl RowOperation for Sum {
        fn operate(&self, m: &mut Matrix) -> Result<()> {
            check_index(self.r1, m.r)?;
            check_index(self.r2, m.r)?;
            for col in 1..=m.c {
                let v1 = m.entry(self.r1, col);
                let v2 = m.entry(self.r2, col);
                m.update(self.r1, col, v1 + v2);
            }
            Ok(())
        }
    }

    impl RowOperation for Multiply {
        fn operate(&self, m: &mut Matrix) -> Result<()> {
            check_index(self.r, m.r)?;
            for col in 1..=m.c {
                let entry = m.entry(self.r, col);
                m.update(self.r, col, entry * self.co);
            }
            Ok(())
        }
    }

    /// Represents a matrix with `r` rows, `c` columns, and a `Vec` of `f64` arranged in row-major form.
    #[derive(std::fmt::Debug, PartialEq)]
    pub struct Matrix {
        r: usize,
        c: usize,
        entries: Vec<f64>
    }

    impl Matrix {
        /// The main method by which `Matrix` instances are created.
        ///
        /// Takes a number of rows `r`, a number of columns `c`, and a list of `entries` for the matrix.
        /// This function can be kind of clunky due to the fact that it requires a `Vec` of `f64`; use [macro::mat] instead, as it converts from integers to floats for you.
        pub fn create(r: usize, c: usize, entries: Vec<f64>) -> Result<Matrix> {
            if r * c == entries.len() {
                Ok(Matrix {
                    r,
                    c,
                    entries
                })
            } else {
                Err(MatrixError { m: format!("Number of specified rows ({:?}) and columns ({:?}) does not match with number of entries ({:?}, should be {:?})", r, c, entries.len(), r * c) })
            }
        }

        /// Creates a matrix with `r` rows and `c` columns, filled with `f64` value `n`.
        ///
        /// # Example
        ///
        /// ```rust
        /// # use calc::{matrix::Matrix, mat};
        /// # #[macro_use]
        /// # fn main() {
        ///
        /// let mut matrix = Matrix::new(2, 3, 5.4);
        /// assert_eq!(matrix, mat!(2; 3; [5.4,5.4,5.4,5.4,5.4,5.4]));
        /// # }
        /// ```
        pub fn new(r: usize, c: usize, n: f64) -> Result<Matrix> {
            Matrix::create(r, c, vec![n; r * c])
        }

        /// Creates an identity matrix of size `s`; that is, a square matrix where all entries are 0, except for those along the main diagonal.
        ///
        /// # Example
        ///
        /// ```rust
        /// # use calc::{matrix::Matrix, mat};
        /// # #[macro_use]
        /// # fn main() {
        ///
        /// let mut matrix = Matrix::identity(3);
        /// assert_eq!(matrix, mat!(3; 3; [1,0,0,0,1,0,0,0,1]))
        /// # }
        /// ```
        pub fn identity(s: usize) -> Result<Matrix> {
            let mut entries = vec![0.0; s * s];
            for i in 0..s {
                entries[i * (s + 1)] = 1.0;
            }
            Matrix::create(s, s, entries)
        }

        /// Creates an elementary matrix of size `s`; that is, an identity matrix that has had one `RowOp` applied to it.
        ///
        /// # Example
        ///
        /// ```rust
        /// # use calc::{matrix::{Sum, Matrix}, mat};
        /// # #[macro_use]
        /// # fn main() {
        ///
        /// let operation = Sum { r1: 1, r2: 3 };
        /// let mut elem = Matrix::elementary(3, operation);
        /// assert_eq!(elem, mat!(3; 3; [1,0,1,
        ///                              0,1,0,
        ///                              0,0,1]));
        /// # }
        /// ```
        pub fn elementary<T: RowOperation>(s: usize, op: T) -> Result<Matrix> {
            let mut m = Matrix::identity(s)?;
            op.operate(&mut m)?;
            Ok(m)
        }

        /// Gets an entry at row `r` and column `c` from the matrix.
        /// Because matrices index from 1, both `r` and `c` are expected to be greater than 0.
        ///
        /// # Example
        ///
        /// ```rust
        /// # use calc::{matrix::Matrix, mat};
        /// # #[macro_use]
        /// # fn main() {
        ///
        /// let matrix = mat!(2; 2; [1, 2, 3, 4]);
        /// assert_eq!(matrix.entry(2, 1), 3.0);
        /// # }
        /// ```
        pub fn entry(&self, r: usize, c: usize) -> f64 {
            self.entries[((r - 1) * self.c) + c - 1] //faster and easier than iter bullshit
        }

        /// Updates an entry at row `r` and column `c` in the matrix to have value `data`.
        /// Because matrices index from 1, both `r` and `c` are expected to be greater than 0.
        ///
        /// # Example
        ///
        /// ```rust
        /// # use calc::{matrix::Matrix, mat};
        /// # #[macro_use]
        /// # fn main() {
        ///
        /// let mut matrix = mat!(2; 2; [1, 3, 5, 7]);
        /// assert_eq!(matrix.entry(1, 2), 3.0);
        /// matrix.update(1, 2, 0.0);
        /// assert_eq!(matrix.entry(1, 2), 0.0);
        /// # }
        /// ```
        pub fn update(&mut self, r: usize, c: usize, data: f64) {
            self.entries[((r - 1) * self.c) + c - 1] = data
        }

        /// Gets the `Matrix`'s vector of entries in row-major form.
        ///
        /// **NOTE**: Because this uses `clone()`, this can be an expensive operation with large matrices.
        ///
        /// # Example
        ///
        /// ```rust
        /// # use calc::{matrix::Matrix, mat};
        /// # #[macro_use]
        /// # fn main() {
        ///
        /// let matrix = mat!(2; 2; [1, 2, 3, 4]);
        /// let entries = vec![1.0, 2.0, 3.0, 4.0]; // note that these are all floating-point values
        /// assert_eq!(matrix.list(), entries);
        /// # }
        /// ```
        pub fn list(&self) -> Vec<f64> {
            self.entries.clone()
        }

        /// Applies a row operation to the matrix and returns itself.
        ///
        /// Because it both mutates the caller and the return type is `&mut Self`, this function can be either chained or used in a looped context.
        /// It is *strongly advised* to not attempt to operate on a `Matrix` using a `Vec<RowOp>` by purely using `.iter().map()`, as `op`'s mutating nature makes it a natural candidate for a more traditional `for` loop.
        ///
        /// # Examples
        ///
        /// ## Chaining calls
        ///
        /// ```rust
        /// # use calc::{matrix::*, mat};
        /// # #[macro_use]
        /// # fn main() {
        ///
        /// // Demonstrating with an identity matrix, for clarity's sake
        /// let mut matrix = Matrix::identity(3); // remember that it MUST be a mutable binding
        /// let op1 = RowOp::Swap(Swap { r1: 1, r2: 2 }); // swap rows 1 and 2
        /// let op2 = RowOp::Sum(Sum { r1: 3, r2: 2}); // sum rows 2 and 3 and put into row 3
        /// let op3 = RowOp::Multiply(Multiply { r: 1, co: 3.0 }); // multiply r1 by 3
        /// matrix.op(op1).op(op2).op(op3);
        ///
        /// assert_eq!(matrix, mat!(3; 3; [0,3,0,1,0,0,1,0,1]))
        /// # }
        /// ```
        ///
        /// ## Looping through a `Vec<RowOp>`
        ///
        /// ```rust
        /// # use calc::{matrix::*, mat};
        /// # #[macro_use]
        /// # fn main() {
        ///
        /// let mut matrix = Matrix::identity(3);
        /// let op1 = RowOp::Swap(Swap { r1: 1, r2: 2 }); // swap rows 1 and 2
        /// let op2 = RowOp::Sum(Sum { r1: 3, r2: 2}); // sum rows 2 and 3 and put into row 3
        /// let op3 = RowOp::Multiply(Multiply { r: 1, co: 3.0 }); // multiply r1 by 3
        /// let operations = vec![op1, op2, op3];
        ///
        /// for e in operations.into_iter() {
        ///     matrix.op(e);
        /// }
        ///
        /// assert_eq!(matrix, mat!(3; 3; [0,3,0,1,0,0,1,0,1]))
        /// # }
        /// ```
        pub fn op(&mut self, operation: RowOp) -> Result<&mut Self> {
            operation.operate(self)?;
            Ok(self)
        }

        pub fn transpose(&self) -> Matrix {
            unimplemented!()
        }

        pub fn rref(&self) -> (Matrix, Vec<RowOp>) {
            unimplemented!()
        }

        pub fn submatrix(&self, r1: usize, r2: usize) -> Matrix {
            unimplemented!()
        }
    }
}

#[cfg(test)]
#[macro_use]
mod tests {
    use crate::matrix::*;

    #[test]
    #[should_panic]
    fn bad_matrix() {
        let mat = mat!(2;2;[1,2,3]);
    }

    #[test]
    fn macro_expansion() {
        let mat = mat!(2;2;[1,2,3,4]);
        assert_eq!(mat.list(), vec![1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn swap_rows() {
        let mut mat = mat!(3;3;[1,2,3,4,5,6,7,8,9]);
        let swap = RowOp::Swap(Swap { r1: 3, r2: 1 });
        assert_eq!(*mat.op(swap), mat!(3;3;[7,8,9,4,5,6,1,2,3]))
    }

    #[test]
    fn sum_rows() {
        let mut mat = mat!(3;3;[1,2,3,4,5,6,7,8,9]);
        let sum = RowOp::Sum(Sum { r1: 1, r2: 2 });
        assert_eq!(*mat.op(sum), mat!(3;3;[5,7,9,4,5,6,7,8,9]))
    }

    #[test]
    fn multiply_row() {
        let mut mat = mat!(2;3;[1,2,3,4,5,6]);
        let mult = RowOp::Multiply(Multiply { r: 1, co: 3.0 });
        assert_eq!(*mat.op(mult), mat!(2;3;[3,6,9,4,5,6]))
    }

    #[test]
    fn elementary() {
        assert_eq!(Matrix::elementary(2, Swap { r1: 1, r2: 2 }), mat!(2;2;[0,1,1,0]));
        assert_eq!(Matrix::elementary(2, Sum { r1: 1, r2: 2 }), mat!(2;2;[1,1,0,1]));
        assert_eq!(Matrix::elementary(2, Multiply { r: 1, co: 2.0 }), mat!(2;2;[2,0,0,1]));
    }
}