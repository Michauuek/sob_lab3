
use num_bigint::BigInt;
use num_bigint::{ToBigInt};
use crate::MagicFunctions;

pub struct ZlotoImplementation;

impl MagicFunctions for ZlotoImplementation {
    fn fib(n: usize) -> BigInt {
        (0..n)
            .into_iter()
            .fold((0.to_bigint().unwrap(), 1.to_bigint().unwrap()), 
                        |(a, b), _| (b.clone(), a + b))
            .0
    }

    fn fac(n: usize) -> BigInt {
        (1..n+1)
            .into_iter()
            .fold(1.to_bigint().unwrap(), |a, b| a * b)
    }
}