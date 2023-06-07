
use num_bigint::BigInt;
use num_bigint::{ToBigInt};
use crate::MagicFunctions;

pub struct RzemiImplementation;

impl MagicFunctions for RzemiImplementation {
    fn fib(n: usize) -> BigInt {
        if n < 2 {
            1.to_bigint().unwrap()
        }else{
            Self::fib(n - 2) + Self::fib(n - 1)
        }
    }

    fn fac(n: usize) -> BigInt {
        if n < 2 {
            1.to_bigint().unwrap()
        } else {
            (n * Self::fac(n - 1)).into()
        }
    }
}