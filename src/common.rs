use num_bigint::BigInt;
use crate::MagicFunctions;
use std::collections::HashMap;
use std::thread;

use crate::swist::SwistImplementation;
use crate::zloto::ZlotoImplementation;
use crate::rzemi::RzemiImplementation;

pub struct FaultTolerantMagicFunctions;

impl MagicFunctions for FaultTolerantMagicFunctions {
    fn fib(n: usize) -> BigInt {
        // Spawn threads for each implementation
        let results = [
            thread::spawn(move || SwistImplementation::fib(n)),
            thread::spawn(move || ZlotoImplementation::fib(n)),
            thread::spawn(move || RzemiImplementation::fib(n)),
        ];
    
        // Collect the results from each thread
        let mut counts: HashMap<BigInt, usize> = HashMap::new();

        
        for result in results {
            let value = result.join().unwrap();
            *counts.entry(value.clone()).or_insert(0) += 1;
        }
        
        dbg!(&counts);

        // Find the most common result
        let mut sorted_counts: Vec<(BigInt, usize)> = counts.into_iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

        match sorted_counts.len() {
            1 | 2 => sorted_counts[0].0.clone(),
            _ => panic!("All implementations returned different results: {:?}", sorted_counts),
        }
    }


    fn fac(n: usize) -> BigInt {
        // Spawn threads for each implementation
        let results = [
            thread::spawn(move || SwistImplementation::fac(n)),
            thread::spawn(move || ZlotoImplementation::fac(n)),
            thread::spawn(move || RzemiImplementation::fac(n)),
        ];
    
        // Collect the results from each thread
        let mut counts: HashMap<BigInt, usize> = HashMap::new();
    
        for result in results {
            let value = result.join().unwrap();
            *counts.entry(value.clone()).or_insert(0) += 1;
        }

        dbg!(&counts);

    
        // Find the most common result
        let mut sorted_counts: Vec<(BigInt, usize)> = counts.into_iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));
    
        match sorted_counts.len() {
            1 | 2 => sorted_counts[0].0.clone(),
            _ => panic!("All implementations returned different results"),
        }
    }
}