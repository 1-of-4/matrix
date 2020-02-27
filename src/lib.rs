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

    pub trait RowOp {
        fn operate(&self, m: &mut Matrix) -> &Matrix;
        fn elementary(&self, s: usize) -> Matrix {
            let mut m = Matrix::identity(s);
            m.op(self);
            m
        }
    }

    pub struct Swap {
        r1: usize,
        r2: usize
    }

    pub struct Sum {
        r1: usize,
        r2: usize
    }

    pub struct Multiply {
        r: usize,
        co: f64
    }

    impl RowOp for Swap {
        fn operate(&self, m: &mut Matrix) -> &Matrix {
            for col in 1..=m.c {
                let v1 = m.entry(self.r1, col);
                let v2 = m.entry(self.r2, col);
                m.update(self.r1, col, v2);
                m.update(self.r2, col, v1);
            }
            m
        }
    }

    impl RowOp for Sum {
        fn operate(&self, m: &mut Matrix) -> &Matrix {
            for col in 1..m.c {
                let v1 = m.entry(self.r1, col);
                let v2 = m.entry(self.r2, col);
                m.update(self.r1, col, v1+v2);
            }
            m
        }
    }

    impl RowOp for Multiply {
        fn operate(&self, m: &mut Matrix) -> &Matrix {
            unimplemented!()
        }
    }

    pub struct Matrix {
        r: usize,
        c: usize,
        entries: Vec<f64>
    }

    impl Matrix {

        pub fn from(r: usize, c: usize, entries: Vec<f64>) -> Matrix { //todo: validate size
            Matrix {
                r,
                c,
                entries
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

        pub fn entry(&self, r: usize, c: usize) -> f64 {
            self.entries[((r-1) * self.c) + c-1] //faster and easier than iter bullshit
        }

        pub fn update(&mut self, r: usize, c: usize, data: f64) {
            self.entries[((r-1) * self.c) + c-1] = data
        }

        pub fn list(&self) -> Vec<f64> {
            self.entries.clone()
        }

        pub fn op<T: RowOp>(&mut self, op: T) -> &Self {
            op.operate(self)
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


//todo will deal with tests later.

//#[cfg(test)]
//#[macro_use]
//mod tests {
//    use crate::matrix::*;
//    #[test]
//    fn macro_expansion() {
//        let mat = mat!(2; 2; [1,2,3,4]);
//        assert_eq!(mat.list(), vec![1.0,2.0,3.0,4.0]);
//    }
//
//    #[test]
//    fn from_array() {
//        let mat = mat!(2;2;[1, 2, 3, 4]);
//        assert_eq!(mat.entry(1, 1), 1.0);
//        assert_eq!(mat.entry(1, 2), 2.0);
//        assert_eq!(mat.entry(2, 1), 3.0);
//        assert_eq!(mat.entry(2, 2), 4.0);
//    }
//
//    #[test]
//    fn swap_rows() {
//        let mut mat = mat!(3;3;[1,2,3,4,5,6,7,8,9]);
//        let swap = Op::Swap::new(3, 1, 3);
//        assert_eq!(mat.op(swap).list(), vec![7, 8, 9,
//                                             4, 5, 6,
//                                             1, 2, 3].iter().map(|e| *e as f64).collect::<Vec<f64>>());
//    }
//
//    #[test]
//    fn sum_rows() {
//        let mut mat = mat!(3;3;[1,2,3,4,5,6,7,8,9]);
//        let sum = Op::Sum::new(3, 1, 2);
//        assert_eq!(mat.op(sum).list(), vec![5, 7, 9,
//                                            4, 5, 6,
//                                            7, 8, 9].iter().map(|e| *e as f64).collect::<Vec<f64>>());
//    }
//}
