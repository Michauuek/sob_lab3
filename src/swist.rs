
use num_bigint::BigInt;
use num_bigint::{ToBigInt};
use crate::MagicFunctions;

pub struct SwistImplementation;

impl MagicFunctions for SwistImplementation {
    fn fib(n: usize) -> BigInt {
        if n == 0 || n == 1 {
            return n.to_bigint().unwrap();
        }

        let mut a = 0.to_bigint().unwrap();
        let mut b = 1.to_bigint().unwrap();

        for _ in 2..=n {
            let tmp = a;
            a = b.clone();
            b += tmp;
        }

        b
    }

    fn fac(n: usize) -> BigInt {
        if n == 0 || n == 1 {
            return 1.to_bigint().unwrap();
        }

        let mut result = 1.to_bigint().unwrap();

        for i in 2..=n {
            result *= i.to_bigint().unwrap();
        }

        result
    }
}