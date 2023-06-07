
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
    println!("{}", common::FaultTolerantMagicFunctions::fib(110));
    println!("{}", common::FaultTolerantMagicFunctions::fac(110));


}
