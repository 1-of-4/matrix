pub mod matrix {
    use std::ops::Deref;

    fn v(r: usize, c: usize) -> Vec<i32> {
        vec![0; r*c]
    }

    pub struct Entry {
        row: usize,
        col: usize,
        data: i32
    }

    impl Entry {
        pub fn new(row: usize, col: usize, data: i32) -> Entry {
            Entry {
                row,
                col,
                data
            }
        }

        pub fn update(&mut self, data: i32) {
            self.data = data;
        }
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
        entries: Vec<Entry>
    }

    impl Matrix {

        pub fn from(r: usize, c: usize, entries: Vec<i32>) -> Matrix {
            Matrix {
                r,
                c,
                entries: entries.iter()
                    .enumerate()
                    .map(
                        |(i,e)|
                            Entry::new(
                                (i%c)+1,
                                (i%r)+1,
                                e.clone()
                            )
                    )
                    .collect()
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

        pub fn entry(&self, r: usize, c: usize) -> &Entry {
            &self.entries[((r - 1) * self.c) + c - 1] //faster and easier than iter bullshit
        }

        pub fn row(&self, r: usize) -> Vec<(usize, &Entry)> { //todo: validate that r < num rows
            self.entries
                .iter()
                .enumerate()
                .filter(|(_,e)| e.row == r)
                .collect()
        }

        pub fn col(&self, c: usize) -> Vec<(usize, &Entry)> { //todo: validate that c < num cols
            self.entries
                .iter()
                .enumerate()
                .filter(|(_,e)| e.col == c)
                .collect()
        }

        pub fn list(&self) -> Vec<i32> {
            self.entries
                .iter()
                .map(|e| e.data)
                .collect()
        }

        fn swap(&mut self, c1: (usize, usize), c2: (usize, usize)) {
            let mut entry1 = *self.entry(c1.0, c1.1);
            let mut entry2 = *self.entry(c2.0, c2.1);
            let temp = entry1.data;
            entry1.update(entry2.data);
            entry2.update(temp);
        }

        pub fn transpose(&self) -> Matrix {
            unimplemented!()
        }

        pub fn row_op(self, op: RowOp) -> Matrix {
            let mut copy = Matrix::from(self.r, self.c, self.list());
            match op.operation {
                Operation::Swap => {
                    for col in 0..copy.c {
                        let entry1 = (op.row1, col);
                        let entry2 = (op.row2, col);
                        copy.swap(entry1, entry2);
                    }
                    copy
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
