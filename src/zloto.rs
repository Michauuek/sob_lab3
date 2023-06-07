
use num_bigint::BigInt;
use num_bigint::{ToBigInt};
use crate::MagicFunctions;

pub struct ZlotoImplementation;

impl MagicFunctions for ZlotoImplementation {
    fn fib(n: usize) -> BigInt {
        return 0.to_bigint().unwrap()
    }

    fn fac(n: usize) -> BigInt {
        return 0.to_bigint().unwrap()
    }
}