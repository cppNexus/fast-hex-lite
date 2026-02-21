use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput};
use std::hint::black_box;
use std::time::Duration;

// ── Helpers ──────────────────────────────────────────────────────────────

type MakeHexFn = fn(&[u8]) -> Vec<u8>;

fn bytes_to_hex_lower(src: &[u8]) -> Vec<u8> {
    let mut hex_buf = vec![0u8; src.len() * 2];
    // Use the reference implementation to generate *valid* lowercase ASCII hex.
    // This keeps decode/decode_in_place benches honest and avoids coupling to fast-hex-lite encode.
    hex::encode_to_slice(src, &mut hex_buf).unwrap();
    hex_buf
}

fn make_hex_lower(lower_hex: &[u8]) -> Vec<u8> {
    lower_hex.to_vec()
}

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

fn bytes_to_hex_mixed_from_lower(lower_hex: &[u8]) -> Vec<u8> {
    // Alternating upper/lower to stress mixed-case handling.
    lower_hex
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if i % 2 == 0 {
                c.to_ascii_uppercase()
            } else {
                c
            }
        })
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

    fn configure(
        self,
        group: &mut criterion::BenchmarkGroup<'_, criterion::measurement::WallTime>,
    ) {
        // Less jitter: longer measurement for tiny inputs, fewer samples for huge inputs.
        match self {
            SizeClass::Small => {
                group.sample_size(80);
                group.warm_up_time(Duration::from_secs(3));
                group.measurement_time(Duration::from_secs(10));
                group.noise_threshold(0.03);
                group.confidence_level(0.99);
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
        "simd"
    } else {
        "scalar"
    }
}

fn batch_size_for_n(n: usize) -> BatchSize {
    // For large inputs, the setup cost (clone) is non-trivial; use LargeInput.
    if n >= 64 * 1024 {
        BatchSize::LargeInput
    } else {
        BatchSize::SmallInput
    }
}

#[inline]
fn is_hex_byte(b: u8) -> bool {
    b.is_ascii_hexdigit()
}

fn validate_hex_ascii(hex: &[u8]) -> bool {
    if (hex.len() & 1) != 0 {
        return false;
    }
    // Tight scalar loop, no allocations, no dst writes.
    hex.iter().copied().all(is_hex_byte)
}

#[inline]
fn debug_assert_valid_hex(hex: &[u8]) {
    // Bench data must be valid hex. If this ever triggers, the generator is wrong or the buffer
    // is being reused after in-place decode.
    debug_assert!(
        validate_hex_ascii(hex),
        "bench input is not valid ASCII hex"
    );
}

// ── Decode benchmarks ─────────────────────────────────────────────────────

fn bench_decode(c: &mut Criterion) {
    for class in [SizeClass::Small, SizeClass::Med, SizeClass::Large] {
        for (case_name, make_hex) in [
            ("lower", make_hex_lower as MakeHexFn),
            ("mixed", bytes_to_hex_mixed_from_lower as MakeHexFn),
        ] {
            let mut group = c.benchmark_group(format!(
                "{}/decode/{}/{case_name}",
                fast_hex_variant(),
                class.name()
            ));
            class.configure(&mut group);

            for &n in class.sizes() {
                let src = make_random_bytes(n);
                let hex_lower = bytes_to_hex_lower(&src);
                let hex = make_hex(&hex_lower);

                debug_assert_valid_hex(&hex);

                // Destination is pre-allocated once per size (no per-iter allocation).
                let mut dst = vec![0u8; n];

                // Throughput base for decode: **decoded output bytes**.
                // This avoids “double counting” (hex input is 2×) and makes results easier to compare.
                group.throughput(Throughput::Bytes(n as u64));

                // fast-hex-lite (scalar/simd)
                group.bench_with_input(BenchmarkId::new("fast-hex-lite", n), &hex, |b, hex_in| {
                    b.iter(|| {
                        let written =
                            fast_hex_lite::decode_to_slice(black_box(hex_in), &mut dst).unwrap();
                        black_box(written);
                    })
                });

                // hex crate (no per-iter allocation)
                group.bench_with_input(BenchmarkId::new("hex-crate", n), &hex, |b, hex_in| {
                    b.iter(|| {
                        hex::decode_to_slice(black_box(hex_in.as_slice()), &mut dst).unwrap();
                        black_box(&dst[..]);
                    })
                });
            }

            group.finish();
        }
    }
}

// ── Validate-only benchmarks (no decode, no dst writes) ──────────────────

fn bench_validate_only(c: &mut Criterion) {
    for class in [SizeClass::Small, SizeClass::Med, SizeClass::Large] {
        for (case_name, make_hex) in [
            ("lower", make_hex_lower as MakeHexFn),
            ("mixed", bytes_to_hex_mixed_from_lower as MakeHexFn),
        ] {
            let mut group = c.benchmark_group(format!(
                "{}/validate_only/{}/{case_name}",
                fast_hex_variant(),
                class.name()
            ));
            class.configure(&mut group);

            for &n in class.sizes() {
                let src = make_random_bytes(n);
                let hex_lower = bytes_to_hex_lower(&src);
                let hex = make_hex(&hex_lower);

                debug_assert_valid_hex(&hex);

                // Throughput base for validate-only: **input hex bytes** (we only read/validate the hex buffer).
                group.throughput(Throughput::Bytes(hex.len() as u64));

                group.bench_with_input(BenchmarkId::new("validate", n), &hex, |b, hex_in| {
                    b.iter(|| {
                        let ok = validate_hex_ascii(black_box(hex_in));
                        black_box(ok);
                    })
                });
            }

            group.finish();
        }
    }
}

// ── Encode benchmarks ─────────────────────────────────────────────────────

fn bench_encode(c: &mut Criterion) {
    for class in [SizeClass::Small, SizeClass::Med, SizeClass::Large] {
        // Lowercase: fair comparison (hex crate only does lowercase).
        {
            let mut group = c.benchmark_group(format!(
                "{}/encode/{}/lower",
                fast_hex_variant(),
                class.name()
            ));
            class.configure(&mut group);

            for &n in class.sizes() {
                let src = make_random_bytes(n);
                let mut dst = vec![0u8; n * 2];

                // Throughput base for encode: **input bytes**.
                group.throughput(Throughput::Bytes(n as u64));

                group.bench_with_input(BenchmarkId::new("fast-hex-lite", n), &src, |b, s| {
                    b.iter(|| {
                        let written =
                            fast_hex_lite::encode_to_slice(black_box(s), &mut dst, true).unwrap();
                        black_box(written);
                    })
                });

                group.bench_with_input(BenchmarkId::new("hex-crate", n), &src, |b, s| {
                    b.iter(|| {
                        hex::encode_to_slice(black_box(s), &mut dst).unwrap();
                        black_box(&dst[..]);
                    })
                });
            }

            group.finish();
        }

        // Uppercase: fast-hex-lite only (hex crate doesn’t provide uppercase-to-slice).
        {
            let mut group = c.benchmark_group(format!(
                "{}/encode/{}/upper",
                fast_hex_variant(),
                class.name()
            ));
            class.configure(&mut group);

            for &n in class.sizes() {
                let src = make_random_bytes(n);
                let mut dst = vec![0u8; n * 2];

                group.throughput(Throughput::Bytes(n as u64));

                group.bench_with_input(BenchmarkId::new("fast-hex-lite", n), &src, |b, s| {
                    b.iter(|| {
                        let written =
                            fast_hex_lite::encode_to_slice(black_box(s), &mut dst, false).unwrap();
                        black_box(written);
                    })
                });
            }

            group.finish();
        }
    }
}

// ── decode_in_place ───────────────────────────────────────────────────────

fn bench_decode_in_place(c: &mut Criterion) {
    // in-place only makes sense when you already have a hex buffer.
    // Clone/setup cost is excluded via `iter_batched_ref`.
    for class in [SizeClass::Small, SizeClass::Med, SizeClass::Large] {
        // Keep it realistic: decode_in_place is mainly interesting for lower-case payloads.
        let mut group = c.benchmark_group(format!(
            "{}/decode_in_place/{}",
            fast_hex_variant(),
            class.name()
        ));
        class.configure(&mut group);

        for &n in class.sizes() {
            // `n` is *decoded* bytes, hex is 2*n bytes.
            let src = make_random_bytes(n);
            let hex = bytes_to_hex_lower(&src);

            debug_assert_valid_hex(&hex);

            // Throughput base for decode_in_place: **decoded output bytes**.
            group.throughput(Throughput::Bytes(n as u64));

            group.bench_with_input(BenchmarkId::new("fast-hex-lite", n), &hex, |b, hex_in| {
                let bs = batch_size_for_n(n);
                b.iter_batched(
                    || hex_in.clone(),
                    |mut buf| {
                        let written = fast_hex_lite::decode_in_place(black_box(&mut buf)).unwrap();
                        black_box(&buf[..written]);
                    },
                    bs,
                )
            });
        }

        group.finish();
    }
}

criterion_group!(
    benches,
    bench_decode,
    bench_validate_only,
    bench_encode,
    bench_decode_in_place
);
criterion_main!(benches);
