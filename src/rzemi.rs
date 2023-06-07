
use std::collections::HashMap;

use num_bigint::BigInt;
use num_bigint::{ToBigInt};
use crate::MagicFunctions;

pub struct RzemiImplementation;

impl MagicFunctions for RzemiImplementation {
    fn fib(n: usize) -> BigInt {
        static mut CACHE: Option<HashMap<usize, BigInt>> = None;
    
        unsafe {
            if CACHE.is_none() {
                CACHE = Some(HashMap::new());
            }
    
            let cache = CACHE.as_mut().unwrap();
    
            if let Some(value) = cache.get(&n) {
                return value.clone();
            }
    
            let result = if n < 2 {
                n.to_bigint().unwrap()
            } else {
                Self::fib(n - 2) + Self::fib(n - 1)
            };
    
            cache.insert(n, result.clone());
            result
        }
    }


    fn fac(n: usize) -> BigInt {
        if n < 2 {
            1.to_bigint().unwrap()
        } else {
            n * Self::fac(n - 1)
        }
    }
}