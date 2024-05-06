use crate::Vector;
use anyhow::{anyhow, Result};
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Mul};

#[allow(dead_code)]
const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

#[allow(dead_code)]
pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}
#[allow(dead_code)]
pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}
#[allow(dead_code)]
pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

#[allow(dead_code)]
impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

#[allow(dead_code)]
impl<T> MsgOutput<T> {
    pub fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}

#[allow(dead_code)]
impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("矩阵维度不匹配 a.col != b.row"));
    }

    let mut result = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                result[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }

    Ok(Matrix {
        data: result,
        row: a.row,
        col: b.col,
    })
}

impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            if i > 0 {
                write!(f, ", ")?;
            }
            for j in 0..self.col {
                if j > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", self.data[i * self.col + j])?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix(row={}, col={}, {})", self.row, self.col, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() -> Result<()> {
        //  1  2    5  6
        //  3  4    7  8
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        let b = Matrix::new([5, 6, 7, 8], 2, 2);
        let result = multiply(&a, &b)?;
        assert_eq!(result.col, 2);
        assert_eq!(result.row, 2);
        assert_eq!(
            format!("{:?}", result),
            "Matrix(row=2, col=2, {19 22, 43 50})"
        );
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        assert_eq!(format!("{:?}", a), "Matrix(row=3, col=2, {1 2, 3 4, 5 6})");
        Ok(())
    }
}
