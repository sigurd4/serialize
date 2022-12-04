#![feature(generic_const_exprs)]

pub trait Serialize
{
    type Output;

    /// Serializes a two-dimensional structure, like `Vec<Vec<_>>`
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// 
    /// let a = [
    ///     [1.0, 2.0],
    ///     [3.0, 4.0]
    /// ];
    /// let s: [f64; 4] = a.serialize();
    /// assert_eq!(s, [1.0, 2.0, 3.0, 4.0]);
    /// ```
    fn serialize(self) -> Self::Output;
}

impl<T> Serialize for &[Vec<T>]
where
    T: Clone
{
    type Output = Vec<T>;
    fn serialize(self) -> Self::Output
    {
        let width = self.len();
        let len: usize = self.iter()
            .map(|xi| xi.len())
            .min()
            .unwrap_or(0);
        (0..width*len)
            .map(|i| self[i%width][i/width].clone())
            .collect()
    }
}
impl<T> Serialize for &[&[T]]
where
    T: Clone
{
    type Output = Vec<T>;
    fn serialize(self) -> Self::Output
    {
        let width = self.len();
        let len: usize = self.iter()
            .map(|xi| xi.len())
            .min()
            .unwrap_or(0);
        (0..width*len)
            .map(|i| self[i%width][i/width].clone())
            .collect()
    }
}

impl<T, const W: usize> Serialize for &[Vec<T>; W]
where
    T: Clone
{
    type Output = Vec<T>;
    fn serialize(self) -> Self::Output
    {
        let len: usize = self.iter()
            .map(|xi| xi.len())
            .min()
            .unwrap_or(0);
        (0..W*len)
            .map(|i| self[i%W][i/W].clone())
            .collect()
    }
}
impl<T, const W: usize> Serialize for &[&[T]; W]
where
    T: Clone
{
    type Output = Vec<T>;
    fn serialize(self) -> Self::Output
    {
        let len: usize = self.iter()
            .map(|xi| xi.len())
            .min()
            .unwrap_or(0);
        (0..W*len)
            .map(|i| self[i%W][i/W].clone())
            .collect()
    }
}

impl<T, const L: usize> Serialize for &[[T; L]]
where
    T: Clone
{
    type Output = Vec<T>;
    fn serialize(self) -> Self::Output
    {
        let width = self.len();
        (0..width*L)
            .map(|i| self[i%width][i/width].clone())
            .collect()
    }
}

impl<T, const L: usize, const W: usize> Serialize for &[[T; L]; W]
where
    T: Clone,
    [(); L*W]:
{
    type Output = [T; L*W];
    fn serialize(self) -> Self::Output
    {
        array_init::array_init(|i| self[i%W][i/W].clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::Serialize;

    #[test]
    fn it_works() {
        let a = [
            [1.0, 2.0],
            [3.0, 4.0]
        ];
        let s: [f64; 4] = a.serialize();

        println!("{}", s.map(|s| s.to_string()).join(", "))
    }
}
