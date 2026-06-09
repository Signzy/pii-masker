use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use pii_masker::Engine;

fn bench_mask(c: &mut Criterion) {
    let engine = Engine::builder().build().expect("engine build failed");
    let clean_line =
        "2024-01-01T12:00:00Z INFO user logged in from 10.20.30.40 with session abc123";
    let pii_line =
        "2024-01-01T12:00:00Z INFO card 4111 1111 1111 1111 ip 10.0.0.1 email user@example.com";

    let mut g = c.benchmark_group("mask");
    g.throughput(Throughput::Bytes(pii_line.len() as u64));

    g.bench_function("clean_line", |b| b.iter(|| engine.mask(black_box(clean_line))));
    g.bench_function("pii_line", |b| b.iter(|| engine.mask(black_box(pii_line))));
    g.finish();
}

criterion_group!(benches, bench_mask);
criterion_main!(benches);
