use anyhow::Result;
use std::ops::{Add, AddAssign, Deref, Mul};
pub struct Vector<T> {
    data: Vec<T>,
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T> + AddAssign,
{
    if a.len() != b.len() {
        anyhow::bail!("Dot product err: a.len != b.len");
    }
    let mut result = T::default();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    Ok(result)
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dot_product() {
        let a = Vector::new([1, 2, 3]);
        let b = Vector::new([4, 5, 6]);
        assert_eq!(dot_product(a, b).unwrap(), 32);
    }
}
