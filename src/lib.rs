pub mod matrix {

    fn v(r: usize, c: usize) -> Vec<i32> {
        vec![0; r*c]
    }

    pub struct Matrix {
        r: usize,
        c: usize,
        entries: Vec<i32>
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

    impl Matrix {

        pub fn list(&self) -> &Vec<i32> {
            &self.entries
        }

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
            self.entries[((r - 1) * self.c) + c - 1]
        }

        pub fn row(&self, r: usize) -> Vec<(usize, i32)> { //todo: validate that r < num rows
            let start = (r-1) * self.c;
            let end = r * self.c;
            self.entries
                .clone() //todo: get rid of this cancer
                .into_iter()
                .enumerate()
                .filter(|i| i.0 >= start && i.0 < end)
                .collect()
        }

        pub fn col(&self, c: usize) -> Vec<(usize, i32)> { //todo: validate that c < num cols
            self.entries
                .clone() //todo: get rid of this cancer
                .into_iter()
                .enumerate()
                .filter(|i| i.0 % self.c == c-1)
                .collect()
        }

        pub fn transpose(&self) -> Matrix {
            unimplemented!()
        }

        pub fn row_op(self, op: RowOp) -> Matrix {
            match op.operation {
                Operation::Swap => {
                    let mut copy = self.entries.clone();
                    let r1 = self.row(op.row1);
                    let r2 = self.row(op.row2);
                    for i in 0..self.c {
                        copy[r1[i].0] = r2[i].1;
                        copy[r2[i].0] = r1[i].1;
                    }
                    Matrix::from(self.r, self.c, copy)
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
        assert_eq!(identity.list(), &vec![1,0,
                                          0,1]);
        let identity = Matrix::identity(3);
        assert_eq!(identity.list(), &vec![1,0,0,
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
    fn get_row() {
        let vec = vec![2,4,6,8];
        let mat = Matrix::from(2,2, vec);
        let r1 = mat.row(1);
        assert_eq!(r1.iter().map(|t| t.1).collect::<Vec<i32>>(), vec![2,4]);
        let r2 = mat.row(2);
        assert_eq!(r2, vec![(2,6),(3,8)])
    }

    #[test]
    fn get_col() {
        let vec = vec![1,3, 5,7];
        let mat = Matrix::from(2,2, vec);
        let c1 = mat.col(1);
        assert_eq!(c1, vec![(0,1), (2,5)])
    }

    #[test]
    fn swap_rows() {
        let matrix = Matrix::from(2,2, vec![1,2,3,4]);
        let original_r1 = matrix.row(1);
        let swap = RowOp::new(Operation::Swap, 1, 2, 0);
        let matrix = matrix.row_op(swap);
        let new_r2 = matrix.row(2);
        assert_eq!(original_r1, new_r2)
    }
}
