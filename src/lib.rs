pub mod matrix {

    fn v(r: usize, c: usize) -> Vec<i32> {
        vec![0; r*c]
    }

    pub enum Operation {
        Swap,
        Sum,
        Multiply
    }

    pub struct RowOp {
        operation: Operation,
        row1: usize,
        row2: usize,
        coefficient: usize
    }

    impl RowOp {
        pub fn new(operation: Operation, row1: usize, row2: usize, coefficient: usize) -> RowOp {
            RowOp {
                operation,
                row1,
                row2,
                coefficient
            }
        }

        pub fn elementary(self, size: usize) -> Matrix {
            let mut m = Matrix::identity(size);
            m.row_op(self);
            m
        }
    }

    pub struct Matrix {
        r: usize,
        c: usize,
        entries: Vec<i32>
    }

    impl Matrix {

        pub fn from(r: usize, c: usize, entries: Vec<i32>) -> Matrix {
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
                entries[i*(s+1)] = 1;
            }
            Matrix::from(s, s, entries)
        }

        pub fn entry(&self, r: usize, c: usize) -> i32 {
            self.entries[((r-1) * self.c) + c-1] //faster and easier than iter bullshit
        }

        pub fn update(&mut self, r: usize, c: usize, data: i32) {
            self.entries[((r-1) * self.c) + c-1] = data
        }

        pub fn list(&self) -> Vec<i32> {
            self.entries.clone()
        }

        pub fn transpose(&self) -> Matrix {
            unimplemented!()
        }

        pub fn row_op(&mut self, op: RowOp) {
            match op.operation {
                Operation::Swap => {
                    for col in 1..=self.c {
                        let v1 = self.entry(op.row1, col);
                        let v2 = self.entry(op.row2, col);
                        self.update(op.row1, col, v2);
                        self.update(op.row2, col, v1);
                    }
                },
                Operation::Sum => unimplemented!(),
                Operation::Multiply => unimplemented!()
            }
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
mod tests {
    use crate::matrix::*;

    #[test]
    fn identity() {
        let identity = Matrix::identity(2);
        assert_eq!(identity.list(), vec![1,0,
                                         0,1]);
        let identity = Matrix::identity(3);
        assert_eq!(identity.list(), vec![1,0,0,
                                         0,1,0,
                                         0,0,1]);
    }

    #[test]
    fn from_array() {
        let vec = vec![1, 2, 3, 4];
        let mat = Matrix::from(2, 2, vec);
        assert_eq!(mat.entry(1, 1), 1);
        assert_eq!(mat.entry(1, 2), 2);
        assert_eq!(mat.entry(2, 1), 3);
        assert_eq!(mat.entry(2, 2), 4);
    }

    #[test]
    fn swap_rows() {
        let mut mat = Matrix::from(3,3, vec![1,2,3,
                                                4,5,6,
                                                7,8,9]);
        let swap = RowOp::new(Operation::Swap, 1, 3, 0);
        mat.row_op(swap);
        assert_eq!(mat.list(), vec![7,8,9,
                                       4,5,6,
                                       1,2,3])
    }
}
