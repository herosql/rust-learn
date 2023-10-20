use std::io::{BufWriter, Write};
use std::net::TcpStream;
use std::ops::Add;

#[derive(Debug)]
struct MyWriter<W> {
    writer: W,
}

impl<W: Write> MyWriter<W> {
    pub fn write(&mut self, buf: &str) -> std::io::Result<()> {
        self.writer.write_all(buf.as_bytes())
    }
}

impl MyWriter<BufWriter<TcpStream>> {
    pub fn new(addr: &str) -> Self {
        let stream = TcpStream::connect(addr).unwrap();
        Self {
            writer: BufWriter::new(stream),
        }
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

// impl Add for &Complex {
// type Output = Self;

// fn add(self, rhs: Self) -> Self::Output {
//     let real = self.real + rhs.real;
//     let imagine = self.imagine + rhs.imagine;
//     Complex::new(real, imagine)
// }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File, io::Write};
    #[test]
    fn test_server() {
        let mut writer = MyWriter::new("127.0.0.1:8080");
        writer.write("hello world!");
    }

    #[test]
    fn test() {
        let mut f = File::create("/tmp/test_write_trait").unwrap();
        // dyn 表达动态分发
        let w: &mut dyn Write = &mut f;
        w.write_all(b"hello").unwrap();
        // let w1 = w.by_ref(); //不能运行,这个函数用不了
        // w1.write_all(b"world").unwrap();
    }

    #[test]
    fn test_complex() {
        let c1 = Complex::new(1.0, 1f64);
        let c2 = Complex::new(1 as f64, 3.0);
        // println!("{:?}", c1 + c2);
    }
}
