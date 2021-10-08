use std::fs::File;
use std::io::{BufReader, Read, Result};

//定义一个带有泛型参数R的reader，不限制R
struct MyReader<R> {
    reader: R,
    buf: String,
}

//实现new方法
impl<R> MyReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf: String::with_capacity(1024),
        }
    }
}

//定义process时，用到R的方法，此时限制R实现Read trait
impl<R> MyReader<R>
where
    R: Read,
{
    pub fn process(&mut self) -> Result<usize> {
        self.reader.read_to_string(&mut self.buf)
    }
}

fn main() {
    let f = File::open("/home/ricardo/.vimrc").unwrap();
    let mut reader = MyReader::new(BufReader::new(f));
    let size = reader.process().unwrap();
    println!("{}", size);
}