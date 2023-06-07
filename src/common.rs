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
    let mut counts: HashMap<Option<BigInt>, usize> = HashMap::new();

    for result in handles {
        let value = result.join().ok();
        *counts.entry(value).or_insert(0) += 1;
    }

    // count the number of nones
    let nones = counts.remove(&None).unwrap_or(0); 

    // Find the most common result
    let mut sorted_counts: Vec<(BigInt, usize)> = counts
    .into_iter()
    .filter_map(|(k, v)| k.map(|k| (k, v)))
    .collect();

    sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

    if nones != 0 {
        println!("Warning: One of the implementations panicked");
    }

    match sorted_counts.len() + nones {
        1 => sorted_counts[0].0.clone(),
        n if n > (N/2+1) => panic!("Rabini są niezdecydowani: {:?}", sorted_counts),
        _ => {
            println!("Warning: One of the implementations returned a different result: {:?}", sorted_counts);
            sorted_counts[0].0.clone()
        }
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