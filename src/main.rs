
use num_bigint::BigInt;
use rzemi::RzemiImplementation;

use crate::{swist::SwistImplementation, zloto::ZlotoImplementation};

mod rzemi;
mod zloto;
mod swist;
mod common;

trait MagicFunctions {
    fn fib(n: usize) -> BigInt;
    fn fac(n: usize) -> BigInt;
}

fn main() {
    println!("{}", RzemiImplementation::fib(6));
    println!("{}", RzemiImplementation::fac(5));
    println!("---------------------------------");
    println!("{}", SwistImplementation::fib(6));
    println!("{}", SwistImplementation::fac(5));
    println!("---------------------------------");
    println!("{}", ZlotoImplementation::fib(6));
    println!("{}", ZlotoImplementation::fac(5));

    println!("---------------------------------");
    println!("{}", RzemiImplementation::fib(30));
    println!("{}", RzemiImplementation::fac(100));
    println!("---------------------------------");
    println!("{}", SwistImplementation::fib(30));
    println!("{}", SwistImplementation::fac(100));
    println!("---------------------------------");
    println!("{}", ZlotoImplementation::fib(30));
    println!("{}", ZlotoImplementation::fac(100));

    println!("---------------------------------");
    println!("{}", common::FaultTolerantMagicFunctions::fib(1000));

    
    // iterate from -10 to 150 with stride 5 and calculate as below

    for i in (1..=150).step_by(5) {
        println!("---------------------------------");
        println!("Calculating for {}", i);
        println!("FIB:");
        println!("{}", common::FaultTolerantMagicFunctions::fib(i));
        println!("FAC:");
        println!("{}", common::FaultTolerantMagicFunctions::fac(i));
    }
}
