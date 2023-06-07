use num_bigint::BigInt;

pub mod rzemi;
pub mod zloto;
pub mod swist;
pub mod common;

pub trait MagicFunctions {
    fn fib(n: usize) -> BigInt;
    fn fac(n: usize) -> BigInt;
}