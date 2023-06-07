
use num_bigint::BigInt;
use rzemi::RzemiImplementation;

mod rzemi;
mod zloto;
mod swist;

trait MagicFunctions {
    fn fib(n: usize) -> BigInt;
    fn fac(n: usize) -> BigInt;
}

fn main() {
    let rzemi = RzemiImplementation;
    RzemiImplementation::fac(2);
}
