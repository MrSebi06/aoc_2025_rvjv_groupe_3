use aoc_2025_rvjv_groupe_3::*;
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
    // D1
    c.bench_function("d1p1", |b| {
        b.iter(|| d1::p1(include_str!("../src/d1/d1.txt")))
    });
    c.bench_function("d1p2", |b| {
        b.iter(|| d1::p2(include_str!("../src/d1/d1.txt")))
    });

    // D2
    c.bench_function("d2p1", |b| {
        b.iter(|| d2::p1(include_str!("../src/d2/d2.txt")))
    });
    c.bench_function("d2p2", |b| {
        b.iter(|| d2::p2(include_str!("../src/d2/d2.txt")))
    });

    // D3
    c.bench_function("d3p1", |b| {
        b.iter(|| d3::p1(include_str!("../src/d3/d3.txt")))
    });
    c.bench_function("d3p2", |b| {
        b.iter(|| d3::p2_mika(include_str!("../src/d3/d3_test.txt")))
    });
    c.bench_function("d3p2", |b| {
        b.iter(|| d3::p2_sacha(include_str!("../src/d3/d3.txt"), 12))
    });
    c.bench_function("d3p2", |b| {
        b.iter(|| d3::p2_seb(include_str!("../src/d3/d3.txt")))
    });

    // D4
    c.bench_function("d4p1", |b| {
        b.iter(|| d4::p1(include_str!("../src/d4/d4.txt")))
    });
    c.bench_function("d4p2", |b| {
        b.iter(|| d4::p2(include_str!("../src/d4/d4.txt")))
    });

    // D5
    c.bench_function("d5p1", |b| {
        b.iter(|| d5::p1(include_str!("../src/d5/d5.txt")))
    });
    c.bench_function("d5p2", |b| {
        b.iter(|| d5::p2(include_str!("../src/d5/d5.txt")))
    });

    // D6
    c.bench_function("d6p1", |b| {
        b.iter(|| d6::p1(include_str!("../src/d6/d6.txt")))
    });
    c.bench_function("d6p2", |b| {
        b.iter(|| d6::p2(include_str!("../src/d6/d6.txt")))
    });

    // D7
    c.bench_function("d7p1", |b| {
        b.iter(|| d7::p1(include_str!("../src/d7/d7.txt")))
    });
    c.bench_function("d7p2", |b| {
        b.iter(|| d7::p2(include_str!("../src/d7/d7.txt")))
    });

    // D8
    c.bench_function("d8p1", |b| {
        b.iter(|| d8::p1(include_str!("../src/d8/d8.txt"), 1000))
    });
    c.bench_function("d8p2", |b| {
        b.iter(|| d8::p2(include_str!("../src/d8/d8.txt")))
    });

    // D9
    c.bench_function("d9p1", |b| {
        b.iter(|| d9::p1(include_str!("../src/d9/d9.txt")))
    });
    c.bench_function("d9p2", |b| {
        b.iter(|| d9::p2(include_str!("../src/d9/d9.txt")))
    });

    // D10
    c.bench_function("d10p1", |b| {
        b.iter(|| d10::p1(include_str!("../src/d10/d10.txt")))
    });
    c.bench_function("d10p2", |b| {
        b.iter(|| d10::p2(include_str!("../src/d10/d10.txt")))
    });

    // D11
    c.bench_function("d11p1", |b| {
        b.iter(|| d11::p1(include_str!("../src/d11/d11.txt")))
    });
    c.bench_function("d11p2", |b| {
        b.iter(|| d11::p2(include_str!("../src/d11/d11.txt")))
    });

    // D12
    c.bench_function("d12p1", |b| {
        b.iter(|| d12::p1(include_str!("../src/d12/d12.txt")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
