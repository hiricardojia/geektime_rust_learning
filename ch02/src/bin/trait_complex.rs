use std::ops::Add;


#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self {
            real,
            imagine,
        }
    }
}

impl Add for Complex {
    type Output = Self;

    //这里的self所有权转移了
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imag = self.imagine + rhs.imagine;
        Self::new(real, imag)
    }
}



fn main() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2 as f64, 3.0);
    println!("{:?}", c1 + c2);
}