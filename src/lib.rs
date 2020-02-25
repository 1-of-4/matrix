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
            Matrix::identity(size).row_op(self)
        }
    }

    pub struct Matrix {
        r: usize,
        c: usize,
        entries: Vec<i32> //todo: put in box?
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
            self.entries[((r - 1) * self.c) + c - 1] //faster and easier than iter bullshit
        }

        pub fn update(&mut self, r: usize, c: usize, data: i32) {
            self.entries[((r-1) * self.c) + c - 1] = data
        }

        pub fn list(&self) -> Vec<i32> {
            self.entries.clone()
        }

        pub fn transpose(&self) -> Matrix {
            unimplemented!()
        }

        pub fn row_op(&mut self, op: RowOp) -> &Self {
            match op.operation {
                Operation::Swap => {
                    for col in 0..self.c {
                        let temp = self.entry(op.row1, col);
                        self.update(op.row1, col, self.entry(op.row2, col));
                        self.update(op.row2, col, temp);
                    }
                    self
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
        let mut matrix = Matrix::from(2,2, vec![1,2,3,4]);
        let original_r1 = matrix.row(1);
        let swap = RowOp::new(Operation::Swap, 1, 2, 0);
        let matrix = matrix.row_op(swap);
        let new_r2 = matrix.row(2);
        assert_eq!(original_r1, new_r2)
    }
}
