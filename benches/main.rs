use criterion::{criterion_group, criterion_main, Criterion, black_box};

use sob_lab::{swist::SwistImplementation, zloto::ZlotoImplementation, rzemi::RzemiImplementation, common::FaultTolerantMagicFunctions};
use sob_lab::MagicFunctions;

fn rzemi_fib_benchmark(c: &mut Criterion) {
    c.bench_function("Rzemi fib(30)", |b| b.iter(|| RzemiImplementation::fib(black_box(30))));
}

fn zlot_fib_benchmark(c: &mut Criterion) {
    c.bench_function("Zloto fib(30)", |b| b.iter(|| ZlotoImplementation::fib(black_box(30))));
    // Add more input sizes as needed
}

fn swist_fib_benchmark(c: &mut Criterion) {
    c.bench_function("Swist fib(30)", |b| b.iter(|| SwistImplementation::fib(black_box(30))));
}

fn rzemi_fac_benchmark(c: &mut Criterion) {
    c.bench_function("Rzemi fac(30)", |b| b.iter(|| RzemiImplementation::fac(black_box(30))));
    // Add more input sizes as needed
}

fn zlot_fac_benchmark(c: &mut Criterion) {
    c.bench_function("Zloto fac(30)", |b| b.iter(|| ZlotoImplementation::fac(black_box(30))));
    // Add more input sizes as needed
}

fn swist_fac_benchmark(c: &mut Criterion) {
    c.bench_function("Swist fac(30)", |b| b.iter(|| SwistImplementation::fac(black_box(30))));
    // Add more input sizes as needed
}

fn common_fac_benchmark(c: &mut Criterion) {
    c.bench_function("Common fac(30)", |b| b.iter(|| FaultTolerantMagicFunctions::fac(black_box(30))));
    // Add more input sizes as needed
}

fn common_fib_benchmark(c: &mut Criterion) {
    c.bench_function("Common fib(30)", |b| b.iter(|| FaultTolerantMagicFunctions::fib(black_box(30))));
    // Add more input sizes as needed
}



// Define the benchmark group
criterion_group!(
    benches,
    rzemi_fib_benchmark,
    swist_fib_benchmark,
    zlot_fib_benchmark,
    rzemi_fac_benchmark,
    swist_fac_benchmark,
    zlot_fac_benchmark,
    common_fac_benchmark,
    common_fib_benchmark
);

criterion_main!(benches);