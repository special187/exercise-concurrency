use crate::{dot_product, Vector};
use anyhow::{anyhow, Result};
use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Mul};
use std::sync::mpsc;
use std::{fmt, thread};

const NUM_THREADS: usize = 4;

pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}
pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}
pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col }
    }
}

impl<T> MsgOutput<T> {
    pub fn new(idx: usize, value: T) -> Self {
        Self { idx, value }
    }
}

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

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput::new(msg.input.idx, value)) {
                        eprintln!("send err: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    let result_len = a.row * b.col;
    let mut result = vec![T::default(); result_len];
    let mut receivers = Vec::with_capacity(result_len);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(col);
            let idx = i * b.col + j;
            let (tx, rx) = oneshot::channel();
            if let Err(e) =
                senders[idx % NUM_THREADS].send(Msg::new(MsgInput::new(idx, row, col), tx))
            {
                eprintln!("send err: {:?}", e)
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        result[output.idx] = output.value;
    }

    Ok(Matrix {
        data: result,
        row: a.row,
        col: b.col,
    })
}

impl<T> Mul for Matrix<T>
where
    T: Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).expect("Matrix multiply err")
    }
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
    fn test_matrix_multiply() -> Result<()> {
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
    fn test_matrix_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        assert_eq!(format!("{:?}", a), "Matrix(row=3, col=2, {1 2, 3 4, 5 6})");
        Ok(())
    }

    #[test]
    fn test_matrix_can_not_multiply() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let ret = multiply(&a, &b);
        assert!(ret.is_err());
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_matrix_can_not_multiply_panic() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let _ret = a * b;
    }
}
