
use num_bigint::BigInt;
use rzemi::RzemiImplementation;

use crate::swist::SwistImplementation;

mod rzemi;
mod zloto;
mod swist;

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
    println!("{}", RzemiImplementation::fib(30));
    println!("{}", RzemiImplementation::fac(100));
    println!("---------------------------------");
    println!("{}", SwistImplementation::fib(30));
    println!("{}", SwistImplementation::fac(100));
}
