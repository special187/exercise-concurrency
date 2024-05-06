use anyhow::Result;
use concurrency::{multiply, Matrix};

fn main() -> Result<()> {
    let a = Matrix::new([1, 2, 3, 4], 2, 2);
    let b = Matrix::new([5, 6, 7, 8], 2, 2);
    let ret = multiply(&a, &b)?;
    println!("{:?}", ret);
    Ok(())
}
