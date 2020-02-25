pub mod matrix {
    use syntax::util::map_in_place::MapInPlace;

    fn v(r: usize, c: usize) -> Vec<i32> {
        vec![0; r*c - 1]
    }

    pub struct Matrix {
        r: usize,
        c: usize,
        entries: Vec<i32>
    }

    struct RowOperation {} //todo

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

        pub fn transpose(&self) -> Matrix {
            unimplemented!()
        }

        pub fn entry(&self, r: usize, c: usize) -> i32 {
            self.entries[((r - 1) * self.c) + c - 1]
        }

        pub fn row(&self, i: usize) -> Vec<i32> {
            unimplemented!()
        }

        pub fn col(&self, i: usize) -> Vec<i32> {
            unimplemented!()
        }

        pub fn rref(&self) -> (Matrix, Vec<RowOperation>) {
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
}
