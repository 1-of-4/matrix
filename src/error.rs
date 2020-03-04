use std::error::Error;
use std::fmt;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct MatrixError {
    pub m: String,
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.m)
    }
}

impl Error for MatrixError {}

pub fn check_index(index: usize, limit: usize) -> Result<(), MatrixError> {
    if index < 1 {
        return Err(MatrixError { m: String::from("Matrices index from 1, but an index of 0 was provided.") })
    }
    let ord = index.cmp(&limit);
    match ord {
        Ordering::Greater => Err(MatrixError { m: format!("The given index ({}) is bigger than the maximum index for a matrix of this size ({})", index, limit) }),
        _ => Ok(())
    }
}