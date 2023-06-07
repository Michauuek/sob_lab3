use num_bigint::BigInt;
use crate::MagicFunctions;
use std::collections::HashMap;
use std::thread::{self, JoinHandle};

use crate::swist::SwistImplementation;
use crate::zloto::ZlotoImplementation;
use crate::rzemi::RzemiImplementation;

pub struct FaultTolerantMagicFunctions;

fn vote<const N: usize>(handles: [JoinHandle<BigInt>; N]) -> BigInt {
    // Collect the results from each thread
    let mut counts: HashMap<BigInt, usize> = HashMap::new();

    for result in handles {
        let value = result.join().unwrap();
        *counts.entry(value.clone()).or_insert(0) += 1;
    }

    // Find the most common result
    let mut sorted_counts: Vec<(BigInt, usize)> = counts.into_iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

    match sorted_counts.len() {
        1 => sorted_counts[0].0.clone(),
        2 => {
            println!("Warning: One of the implementations returned a different result: {:?}", sorted_counts);
            sorted_counts[0].0.clone()
        }
        _ => panic!("All implementations returned different results: {:?}", sorted_counts),
    }
}

impl MagicFunctions for FaultTolerantMagicFunctions {
    fn fib(n: usize) -> BigInt {        
        vote([
            thread::spawn(move || SwistImplementation::fib(n)),
            thread::spawn(move || ZlotoImplementation::fib(n)),
            thread::spawn(move || RzemiImplementation::fib(n)),
        ])
    }


    fn fac(n: usize) -> BigInt {
        vote([
            thread::spawn(move || SwistImplementation::fac(n)),
            thread::spawn(move || ZlotoImplementation::fac(n)),
            thread::spawn(move || RzemiImplementation::fac(n)),
        ])
    }
}