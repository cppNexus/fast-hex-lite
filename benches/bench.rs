use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::time::Duration;

// ── Helpers ──────────────────────────────────────────────────────────────

/// Simple deterministic LCG — no external dependency needed.
fn make_random_bytes(n: usize) -> Vec<u8> {
    let mut state: u64 = 0xdeadbeef_cafebabe;
    (0..n)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as u8
        })
        .collect()
}

fn bytes_to_hex_lower(src: &[u8]) -> Vec<u8> {
    let mut hex = vec![0u8; src.len() * 2];
    fast_hex_lite::encode_to_slice(src, &mut hex, true).unwrap();
    hex
}

fn bytes_to_hex_mixed_from_lower(lower_hex: &[u8]) -> Vec<u8> {
    // Alternating upper/lower to stress validation / mixed-case handling.
    lower_hex
        .iter()
        .enumerate()
        .map(|(i, &c)| if i % 2 == 0 { c.to_ascii_uppercase() } else { c })
        .collect()
}

#[derive(Clone, Copy)]
enum SizeClass {
    Small,
    Med,
    Large,
}

impl SizeClass {
    fn name(self) -> &'static str {
        match self {
            SizeClass::Small => "small",
            SizeClass::Med => "med",
            SizeClass::Large => "large",
        }
    }

    fn sizes(self) -> &'static [usize] {
        match self {
            SizeClass::Small => &[32, 256],
            SizeClass::Med => &[4 * 1024],
            SizeClass::Large => &[64 * 1024, 1024 * 1024],
        }
    }

    fn configure(self, group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>) {
        // Fewer outliers / less jitter: longer measurement for tiny inputs,
        // fewer samples for huge inputs.
        match self {
            SizeClass::Small => {
                group.sample_size(80);
                group.warm_up_time(Duration::from_secs(3));
                group.measurement_time(Duration::from_secs(10));
                group.noise_threshold(0.03);
            }
            SizeClass::Med => {
                group.sample_size(60);
                group.warm_up_time(Duration::from_secs(3));
                group.measurement_time(Duration::from_secs(8));
                group.noise_threshold(0.03);
            }
            SizeClass::Large => {
                group.sample_size(30);
                group.warm_up_time(Duration::from_secs(2));
                group.measurement_time(Duration::from_secs(6));
                group.noise_threshold(0.03);
            }
        }
    }
}

fn fast_hex_variant() -> &'static str {
    // Same bench file works for both `cargo bench` and `cargo bench --features simd`.
    // The library chooses scalar vs simd internally under the `simd` feature.
    if cfg!(feature = "simd") {
        "fast-hex-lite/simd"
    } else {
        "fast-hex-lite/scalar"
    }
}

// ── Decode benchmarks ─────────────────────────────────────────────────────

fn bench_decode(c: &mut Criterion) {
    for class in [SizeClass::Small, SizeClass::Med, SizeClass::Large] {
        let mut group = c.benchmark_group(format!("decode/{}", class.name()));
        class.configure(&mut group);

        for &n in class.sizes() {
            let src = make_random_bytes(n);
            let hex_lower = bytes_to_hex_lower(&src);
            let hex_mixed = bytes_to_hex_mixed_from_lower(&hex_lower);

            // Pre-allocate destination for *all* decoders.
            let mut dst = vec![0u8; n];

            // Throughput in decoded bytes.
            group.throughput(Throughput::Bytes(n as u64));

            // fast-hex-lite: lowercase input
            group.bench_with_input(
                BenchmarkId::new(format!("{}/decode/lower", fast_hex_variant()), n),
                &n,
                |b, _| {
                    b.iter(|| {
                        fast_hex_lite::decode_to_slice(black_box(&hex_lower), black_box(&mut dst))
                            .unwrap()
                    })
                },
            );

            // fast-hex-lite: mixed-case input
            group.bench_with_input(
                BenchmarkId::new(format!("{}/decode/mixed", fast_hex_variant()), n),
                &n,
                |b, _| {
                    b.iter(|| {
                        fast_hex_lite::decode_to_slice(black_box(&hex_mixed), black_box(&mut dst))
                            .unwrap()
                    })
                },
            );

            // hex crate: decode into pre-allocated buffer (no per-iter allocation)
            // NOTE: hex::decode_to_slice takes &str, so we build it once.
            let hex_lower_str = core::str::from_utf8(&hex_lower).unwrap();
            let hex_mixed_str = core::str::from_utf8(&hex_mixed).unwrap();

            group.bench_with_input(
                BenchmarkId::new("hex-crate/decode/lower", n),
                &n,
                |b, _| {
                    b.iter(|| {
                        let written = hex::decode_to_slice(black_box(hex_lower_str), black_box(&mut dst)).unwrap();
                        black_box(written)
                    })
                },
            );

            group.bench_with_input(
                BenchmarkId::new("hex-crate/decode/mixed", n),
                &n,
                |b, _| {
                    b.iter(|| {
                        let written = hex::decode_to_slice(black_box(hex_mixed_str), black_box(&mut dst)).unwrap();
                        black_box(written)
                    })
                },
            );
        }

        group.finish();
    }
}

// ── Encode benchmarks ─────────────────────────────────────────────────────

fn bench_encode(c: &mut Criterion) {
    for class in [SizeClass::Small, SizeClass::Med, SizeClass::Large] {
        let mut group = c.benchmark_group(format!("encode/{}", class.name()));
        class.configure(&mut group);

        for &n in class.sizes() {
            let src = make_random_bytes(n);
            let mut dst = vec![0u8; n * 2];

            group.throughput(Throughput::Bytes(n as u64));

            // fast-hex-lite: lower
            group.bench_with_input(
                BenchmarkId::new(format!("{}/encode/lower", fast_hex_variant()), n),
                &n,
                |b, _| {
                    b.iter(|| {
                        fast_hex_lite::encode_to_slice(black_box(&src), black_box(&mut dst), true).unwrap()
                    })
                },
            );

            // fast-hex-lite: upper
            group.bench_with_input(
                BenchmarkId::new(format!("{}/encode/upper", fast_hex_variant()), n),
                &n,
                |b, _| {
                    b.iter(|| {
                        fast_hex_lite::encode_to_slice(black_box(&src), black_box(&mut dst), false).unwrap()
                    })
                },
            );

            // hex crate: encode into pre-allocated buffer (no per-iter allocation)
            group.bench_with_input(
                BenchmarkId::new("hex-crate/encode/lower", n),
                &n,
                |b, _| {
                    b.iter(|| {
                        hex::encode_to_slice(black_box(&src), black_box(&mut dst)).unwrap();
                    })
                },
            );
        }

        group.finish();
    }
}

// ── decode_in_place ───────────────────────────────────────────────────────

fn bench_decode_in_place(c: &mut Criterion) {
    // in-place only makes sense when you already have a hex buffer.
    // keep sizes smaller to avoid huge clone cost dominating.
    for class in [SizeClass::Small, SizeClass::Med, SizeClass::Large] {
        let sizes = match class {
            SizeClass::Small => &[32usize, 256usize][..],
            SizeClass::Med => &[4 * 1024usize][..],
            SizeClass::Large => &[64 * 1024usize][..],
        };

        let mut group = c.benchmark_group(format!("decode_in_place/{}", class.name()));
        class.configure(&mut group);

        for &n in sizes {
            let src = make_random_bytes(n);
            let hex = bytes_to_hex_lower(&src);

            group.throughput(Throughput::Bytes(n as u64));

            group.bench_with_input(
                BenchmarkId::new(format!("{}/decode_in_place", fast_hex_variant()), n),
                &n,
                |b, _| {
                    b.iter(|| {
                        let mut buf = hex.clone();
                        fast_hex_lite::decode_in_place(black_box(&mut buf)).unwrap()
                    })
                },
            );
        }

        group.finish();
    }
}

criterion_group!(benches, bench_decode, bench_encode, bench_decode_in_place);
criterion_main!(benches);