
use num_bigint::BigInt;
use num_bigint::{ToBigInt};
use crate::MagicFunctions;

pub struct ZlotoImplementation;

impl MagicFunctions for ZlotoImplementation {
    fn fib(n: usize) -> BigInt {
        // fibbonaci using golder ratio algorithm
        const PHI: f64 = 1.61803398874989484820458683436563811772030917980576286213544862270526046281890_f64;

        let fib = ((PHI.powi(n as i32) - (-PHI).powi(-(n as i32))) / 5.0_f64.sqrt()).to_bigint().unwrap();

        fib 
    }

    fn fac(n: usize) -> BigInt {
        // factorial using stirling's approximation
        const PI: f64 =  std::f64::consts::PI;
        const E: f64 = std::f64::consts::E;

        let n = n as f64;

        let fac = (n / E).powf(n) * (2.0 * n + (2.0 * n / 3.0).sqrt() + 1.0 / (205.0 * n)).sqrt() * PI.sqrt();

        fac.to_bigint().unwrap()
    }
}