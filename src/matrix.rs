use anyhow::Result;
use std::{
    fmt::{self, Debug},
    ops::{Add, AddAssign, Mul},
    sync::mpsc,
    thread,
};

use crate::dot_product;
use crate::Vector;
const NUM_THREADS: usize = 4;

// [[1,2] [1,2]]   [1,2,1,2] 后面这个形式效率更高

#[derive(PartialEq, Eq)]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

impl<T> Mul for Matrix<T>
where
    T: Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Send + 'static,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        multiply(&self, &rhs).unwrap()
    }
}

#[allow(dead_code)]
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Copy + Send + 'static,
{
    if a.row != b.col {
        return Err(anyhow::anyhow!("Err a.row != b.col  "));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.row, msg.input.col)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        idx: msg.input.idx,
                        value,
                    }) {
                        eprintln!("Send err{:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<_>>();

    // 生成四个线程，线程接受信息，并dot_product

    let matrix_len = a.row * b.col;
    let mut data = vec![T::default(); matrix_len];
    let mut receivers = Vec::with_capacity(matrix_len);
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data: Vec<_> = b.data[j..].iter().step_by(b.col).copied().collect();
            let col = Vector::new(col_data);

            let index = i * b.col + j;
            let input = MsgInput::new(index, row, col);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            //data[i*b.col + j] += dot_product(row, col)?;
            if let Err(e) = senders[index % NUM_THREADS].send(msg) {
                eprintln!("Send err{:?}", e);
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let msg = rx.recv()?;
        data[msg.idx] = msg.value;
    }

    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

impl<T: Debug> Matrix<T> {
    // 这里的data  是只要可以转变为Vec<T>就可以使用
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Matrix<T> {
        Matrix {
            data: data.into(),
            row,
            col,
        }
    }
}

// display a 2x3 {1 2 3
//                1 2 3}
impl<T: Debug> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{:?} ", self.data[i * self.col + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: fmt::Display + Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix {{row: {}, col: {}, data: {} }}",
            self.row, self.col, self,
        )
    }
}

pub struct MsgInput<T> {
    idx: usize,
    row: Vector<T>,
    col: Vector<T>,
}

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> MsgInput<T> {
        MsgInput { idx, row, col }
    }
}
pub struct MsgOutput<T> {
    idx: usize,
    value: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,

    // TODO： 这里不清楚
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Msg<T> {
        Msg { input, sender }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply() -> Result<()> {
        let a: Matrix<i32> = Matrix::new(vec![1, 2, 3, 4, 5, 6], 2, 3);
        let b: Matrix<i32> = Matrix::new(vec![1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
        assert_eq!(c.row, 2);
        assert_eq!(c.col, 2);
        assert_eq!(c.data, [22, 28, 49, 64]);
        Ok(())
    }
}
