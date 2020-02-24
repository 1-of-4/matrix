pub mod matrix {
    pub struct Matrix {
        entries: [[i32; usize]; usize]
    }

    impl Matrix {
        pub fn new(rows: usize, cols: usize) -> Matrix {
            Matrix { entries: [[0; rows]; cols] }
        }

        pub fn identity(size: usize) -> Matrix {
            let mut entries = [[0; size]; size];
            for i in 0..size {
                entries[i][i] = 1;
            }
            Matrix { entries }
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
