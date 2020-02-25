pub mod matrix {

    fn v(r: usize, c: usize) -> Vec<i32> {
        vec![0; r*c - 1]
    }

    pub struct Matrix {
        r: usize,
        c: usize,
        entries: Vec<i32>
    }

    pub enum Operation {
        Swap,
        Sum,
        Reduce
    }

    pub struct RowOperation {
        op: Operation,
        r1: usize,
        r2: Option<usize>,
        co: Option<usize>
    }

    impl RowOperation {
        pub fn new(op: Operation, r1: usize, r2: Option<usize>, co: Option<usize>) -> RowOperation {
            RowOperation {
                op,
                r1,
                r2,
                co
            }
        }

        pub fn elementary(&self, size: usize) -> Matrix {
            Matrix::identity(size).row_op(self)
        }
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

        pub fn identity(size: usize) -> Matrix {
            let mut entries = v(size, size);
            for i in 0..=size {
                entries[i*size - 1] = 1;
            }
            Matrix::from(size, size, entries)
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

        pub fn row_op(&self, row_op: &RowOperation) -> Matrix {
            match row_op.op {
                Operation::Swap => unimplemented!(),
                Operation::Sum => unimplemented!(),
                Operation::Reduce => unimplemented!()
            }
        }

        pub fn rref(&self) -> (Matrix, Vec<RowOperation>) {
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
        let vec = vec![1,3,5,7];
        let mat = Matrix::from(2,2, vec);
        let c1 = mat.col(1);
        assert_eq!(c1, vec![(0,1), (2,5)])
    }
}
