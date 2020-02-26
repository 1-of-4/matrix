//macro_rules! mat {
//    ($($x:expr),*) => (<[_]>::into_vec(Box::new([$($x as f64),*])));
//}

pub mod matrix {

    macro_rules! mat {
        (
            $(
                $cols:expr; [ $($e:expr),* ]
            )?
        ) => {
            Matrix::
        }
    }

    fn v(r: usize, c: usize) -> Vec<f64> {
        vec![0.0; r*c]
    }

    fn fv(v: Vec<i32>) -> Vec<f64> {
        v.iter().map(|e| *e as f64).collect()
    }

    pub enum Operation {
        Swap,
        Sum,
        Multiply
    }

    pub struct RowOp {
        operation: Operation,
        size: usize,
        row1: usize,
        row2: usize,
        coefficient: usize
    }

    impl RowOp {
        pub fn new(operation: Operation, size: usize, row1: usize, row2: usize, coefficient: usize) -> RowOp { //todo: validate
            RowOp {
                operation,
                size,
                row1,
                row2,
                coefficient
            }
        }

        pub fn elementary(&self) -> Matrix {
            let mut m = Matrix::identity(self.size);
            m.row_op(self);
            m
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

        pub fn row_op(&mut self, op: &RowOp) -> &Self {
            match op.operation {
                Operation::Swap => {
                    for col in 1..=self.c {
                        let v1 = self.entry(op.row1, col);
                        let v2 = self.entry(op.row2, col);
                        self.update(op.row1, col, v2);
                        self.update(op.row2, col, v1);
                    }
                },
                Operation::Sum => {
                    for col in 1..self.c {
                        let v1 = self.entry(op.row1, col);
                        let v2 = self.entry(op.row2, col);
                        self.update(op.row1, col, v1+v2);
                    }
                },
                Operation::Multiply => unimplemented!()
            }
            self
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
    fn macro_expansion() {
        assert_eq!(mat![1,2,3], vec![1.0,2.0,3.0]);
    }

//    #[test]
//    fn identity() {
//        let identity = Matrix::identity(2);
//        assert_eq!(identity.list(), vec![1,0,
//                                         0,1]);
//        let identity = Matrix::identity(3);
//        assert_eq!(identity.list(), vec![1,0,0,
//                                         0,1,0,
//                                         0,0,1]);
//    }
//
//    #[test]
//    fn from_array() {
//        let vec = vec![1, 2, 3, 4];
//        let mat = Matrix::from(2, 2, vec);
//        assert_eq!(mat.entry(1, 1), 1);
//        assert_eq!(mat.entry(1, 2), 2);
//        assert_eq!(mat.entry(2, 1), 3);
//        assert_eq!(mat.entry(2, 2), 4);
//    }
//
//    #[test]
//    fn swap_rows() {
//        let original = vec![1,2,3,4,5,6,7,8,9];
//        let mut mat = Matrix::from(3,3, original);
//        let swap = RowOp::new(Operation::Swap, 3, 1, 3, 0);
//        assert_eq!(mat.row_op(&swap).list(), vec![7,8,9,
//                                                  4,5,6,
//                                                  1,2,3]);
//        assert_eq!(swap.elementary().list(), vec![0,0,1,
//                                                  0,1,0,
//                                                  1,0,0])
//    }
//
//    #[test]
//    fn sum_rows() {
//        let original = vec![1,2,3,4,5,6,7,8,9];
//        let mut mat = Matrix::from(3,3, original);
//        let sum = RowOp::new(Operation::Sum, 3, 1, 2, 0);
//        assert_eq!(mat.row_op(&sum).list(), vec![5,7,9,
//                                                 4,5,6,
//                                                 7,8,9]);
//        assert_eq!(sum.elementary().list(), vec![1,1,0,
//                                                 0,1,0,
//                                                 0,0,1])
//    }
}
