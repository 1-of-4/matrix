#[macro_use]
pub mod matrix {
    #[macro_export]
    macro_rules! mat {
        (
            $(
                $r:expr; $c:expr; [$($e:expr),+]
            )?
        ) => {{
            let vec = vec![$($($e as f64),+),*];
            Matrix::from($($r)?, $($c)?, vec)
        }}
    }

    fn v(r: usize, c: usize) -> Vec<f64> {
        vec![0.0; r*c]
    }

    pub struct Swap {
        pub r1: usize,
        pub r2: usize
    }

    pub struct Sum {
        pub r1: usize,
        pub r2: usize
    }

    pub struct Multiply {
        pub r: usize,
        pub co: f64
    }

    pub trait RowOp {
        fn operate(&self, m: &mut Matrix);
    }

    impl RowOp for Swap {
        fn operate(&self, m: &mut Matrix) {
            for col in 1..=m.c {
                let v1 = m.entry(self.r1, col);
                let v2 = m.entry(self.r2, col);
                m.update(self.r1, col, v2);
                m.update(self.r2, col, v1);
            }
        }
    }

    impl RowOp for Sum {
        fn operate(&self, m: &mut Matrix) {
            for col in 1..=m.c {
                let v1 = m.entry(self.r1, col);
                let v2 = m.entry(self.r2, col);
                m.update(self.r1, col, v1+v2);
            }
        }
    }

    impl RowOp for Multiply {
        fn operate(&self, m: &mut Matrix) {
            unimplemented!()
        }
    }

    pub struct Matrix {
        r: usize,
        c: usize,
        entries: Vec<f64>
    }

    impl Matrix {

        pub fn from(r: usize, c: usize, entries: Vec<f64>) -> Matrix {
            if r*c == entries.len() {
                Matrix {
                    r,
                    c,
                    entries
                }
            } else {
                panic!("Number of specified rows ({}) and columns ({}) does not match with number of entries ({}, should be {})", r, c, entries.len(), r*c)
            }
        }

        pub fn new(r: usize, c: usize) -> Matrix {
            Matrix::from(r, c, v(r, c))
        }

        pub fn identity(s: usize) -> Matrix {
            let mut entries = v(s, s);
            for i in 0..s {
                entries[i*(s+1)] = 1.0;
            }
            Matrix::from(s, s, entries)
        }

        pub fn elementary<T: RowOp>(s: usize, op: T) -> Matrix {
            let mut m = Matrix::identity(s);
            op.operate(&mut m);
            m
        }

        pub fn entry(&self, r: usize, c: usize) -> f64 {
            self.entries[((r-1) * self.c) + c-1] //faster and easier than iter bullshit
        }

        pub fn update(&mut self, r: usize, c: usize, data: f64) {
            self.entries[((r-1) * self.c) + c-1] = data
        }

        pub fn list(&self) -> Vec<f64> {
            self.entries.clone()
        }

        pub fn op<T: RowOp>(&mut self, operation: T) {
            operation.operate(self)
        }

        pub fn transpose(&self) -> Matrix {
            unimplemented!()
        }

        pub fn rref<T: RowOp>(&self) -> (Matrix, Vec<T>) {
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
        assert_eq!(mat.list(), vec![1.0,2.0,3.0,4.0]);
    }

    #[test]
    fn from_array() {
        let mat = mat!(2;2;[1, 2, 3, 4]);
        assert_eq!(mat.entry(1, 1), 1.0);
        assert_eq!(mat.entry(1, 2), 2.0);
        assert_eq!(mat.entry(2, 1), 3.0);
        assert_eq!(mat.entry(2, 2), 4.0);
    }

    #[test]
    fn swap_rows() {
        let mut mat = mat!(3;3;[1,2,3,4,5,6,7,8,9]);
        let swap = Swap { r1: 3, r2: 1 };
        mat.op(swap);
        assert_eq!(mat.list(), mat!(3;3;[7,8,9,4,5,6,1,2,3]).list())
    }

    #[test]
    fn sum_rows() {
        let mut mat = mat!(3;3;[1,2,3,4,5,6,7,8,9]);
        let sum = Sum { r1: 1, r2: 2 };
        mat.op(sum);
        assert_eq!(mat.list(), mat!(3;3;[5,7,9,4,5,6,7,8,9]).list())
    }
}
