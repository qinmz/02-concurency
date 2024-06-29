use anyhow::Result;
use concurrency::Matrix;

/// 这里的impl Into是这个意思， 只要可以转成 Vec的都可以 传进来
// impl<T> Matrix<T> {
//     pub fn new (data: impl Into<Vec<T>>, row:usize, col: usize) -> Self {
//         Matrix {
//             data: data.into(),
//             row,
//             col,
//         }
//     }
// }

// impl<T> std::fmt::Display for Matrix<T>
// where
//     T: std::fmt::Display
// {

//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{{")?;
//         for i in 0..self.row {
//             // display a 2x3 as {1 2 3, 4 5 6}
//             for j in 0..self.col {
//                 write!(f, "{}", self.data[i * self.col + j])?;
//                 if j != self.col -1 {
//                     write!(f," ")?;
//                 }
//             }
//             if i != self.row -1 {
//                 write!(f, ",")?;
//             }
//         }
//         write!(f, "}}")?;
//         Ok(())
//     }
// }

// impl<T> Debug for Matrix<T>
// where
//     T: std::fmt::Display,
// {

//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f,"Matrix: row = {}, col = {}, data = {}", self.row, self.col, self)

//     }
// }

fn main() -> Result<()> {
    let m = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
    let n = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
    println!("{}", m * n);
    Ok(())
}
