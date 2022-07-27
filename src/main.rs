use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;
use std::fs::File;
use std::io::{Write, Error};

fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}
fn main() -> Result<(),Error>{
    let path = "lines.txt";
    let mut output = File::create(path)?;
    write!(output, "{}",fib(12648430))?;
    // println!("fib(1000) = {}", fib(12648430));
    Ok(())
}