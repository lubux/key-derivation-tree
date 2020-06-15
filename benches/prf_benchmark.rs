extern crate criterion;
use criterion::{criterion_group, criterion_main, Criterion};
use criterion::BenchmarkId;
use constrained_prf::prf;
use constrained_prf::errors::InvalidAccessError;

fn bench_prf(height: u16) -> Result<[u8; 16], InvalidAccessError> {
    let key = [0u8; 16];
    let prf = prf::PRF::init(height as u16, key);
    prf.apply(0)
}

fn from_elem(c: &mut Criterion) {

    let mut group = c.benchmark_group("PRF derive");
    for height in [2u16, 4u16, 8u16, 16u16, 32u16].iter() {
        group.bench_with_input(BenchmarkId::new("Tree height", height), height, |b, &h| {
            b.iter(|| bench_prf(h));
        });
    }
    group.finish();
}

criterion_group!(benches, from_elem);
criterion_main!(benches);