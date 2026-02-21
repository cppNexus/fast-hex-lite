cargo bench --features simd
   Compiling fast-hex-lite v0.1.0 (/Users/vlad/Coding/Rust/fast-hex-lite)
    Finished `bench` profile [optimized] target(s) in 12.34s
     Running benches/bench.rs (target/release/deps/bench-935ff5f8ddaa0a06)
Gnuplot not found, using plotters backend
Benchmarking scalar/decode/small/lower/fast-hex-lite/32: Collecting 80 samples in estimated 10.000 s (557M iteration
scalar/decode/small/lower/fast-hex-lite/32
                        time:   [17.879 ns 17.895 ns 17.912 ns]
                        thrpt:  [1.6638 GiB/s 1.6654 GiB/s 1.6668 GiB/s]
Found 7 outliers among 80 measurements (8.75%)
  3 (3.75%) high mild
  4 (5.00%) high severe
scalar/decode/small/lower/hex-crate/32
                        time:   [45.619 ns 46.031 ns 46.457 ns]
                        thrpt:  [656.90 MiB/s 662.98 MiB/s 668.96 MiB/s]
Found 7 outliers among 80 measurements (8.75%)
  6 (7.50%) high mild
  1 (1.25%) high severe
Benchmarking scalar/decode/small/lower/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (66M iteration
scalar/decode/small/lower/fast-hex-lite/256
                        time:   [151.36 ns 151.74 ns 152.57 ns]
                        thrpt:  [1.5626 GiB/s 1.5713 GiB/s 1.5752 GiB/s]
Found 10 outliers among 80 measurements (12.50%)
  1 (1.25%) low severe
  5 (6.25%) high mild
  4 (5.00%) high severe
scalar/decode/small/lower/hex-crate/256
                        time:   [382.99 ns 383.59 ns 384.37 ns]
                        thrpt:  [635.16 MiB/s 636.46 MiB/s 637.46 MiB/s]
Found 4 outliers among 80 measurements (5.00%)
  4 (5.00%) high mild

Benchmarking scalar/decode/small/mixed/fast-hex-lite/32: Collecting 80 samples in estimated 10.000 s (558M iteration
scalar/decode/small/mixed/fast-hex-lite/32
                        time:   [17.879 ns 17.908 ns 17.964 ns]
                        thrpt:  [1.6590 GiB/s 1.6642 GiB/s 1.6668 GiB/s]
Found 8 outliers among 80 measurements (10.00%)
  3 (3.75%) high mild
  5 (6.25%) high severe
scalar/decode/small/mixed/hex-crate/32
                        time:   [43.447 ns 43.821 ns 44.286 ns]
                        thrpt:  [689.11 MiB/s 696.42 MiB/s 702.40 MiB/s]
Found 1 outliers among 80 measurements (1.25%)
  1 (1.25%) high mild
Benchmarking scalar/decode/small/mixed/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (66M iteration
scalar/decode/small/mixed/fast-hex-lite/256
                        time:   [151.07 ns 151.24 ns 151.40 ns]
                        thrpt:  [1.5747 GiB/s 1.5764 GiB/s 1.5782 GiB/s]
Found 3 outliers among 80 measurements (3.75%)
  1 (1.25%) high mild
  2 (2.50%) high severe
scalar/decode/small/mixed/hex-crate/256
                        time:   [346.10 ns 348.60 ns 351.67 ns]
                        thrpt:  [694.23 MiB/s 700.34 MiB/s 705.41 MiB/s]
Found 5 outliers among 80 measurements (6.25%)
  4 (5.00%) high mild
  1 (1.25%) high severe

Benchmarking scalar/decode/med/lower/fast-hex-lite/4096: Collecting 60 samples in estimated 8.0008 s (3.6M iteration
scalar/decode/med/lower/fast-hex-lite/4096
                        time:   [2.2395 µs 2.2451 µs 2.2550 µs]
                        thrpt:  [1.6916 GiB/s 1.6992 GiB/s 1.7033 GiB/s]
Found 6 outliers among 60 measurements (10.00%)
  3 (5.00%) high mild
  3 (5.00%) high severe
scalar/decode/med/lower/hex-crate/4096
                        time:   [6.5168 µs 6.5437 µs 6.5817 µs]
                        thrpt:  [593.50 MiB/s 596.94 MiB/s 599.42 MiB/s]

Benchmarking scalar/decode/med/mixed/fast-hex-lite/4096: Collecting 60 samples in estimated 8.0017 s (3.6M iteration
scalar/decode/med/mixed/fast-hex-lite/4096
                        time:   [2.2400 µs 2.2434 µs 2.2492 µs]
                        thrpt:  [1.6960 GiB/s 1.7004 GiB/s 1.7030 GiB/s]
Found 9 outliers among 60 measurements (15.00%)
  2 (3.33%) high mild
  7 (11.67%) high severe
scalar/decode/med/mixed/hex-crate/4096
                        time:   [6.2453 µs 6.2925 µs 6.3455 µs]
                        thrpt:  [615.59 MiB/s 620.78 MiB/s 625.47 MiB/s]
Found 9 outliers among 60 measurements (15.00%)
  7 (11.67%) low severe
  1 (1.67%) low mild
  1 (1.67%) high mild

Benchmarking scalar/decode/large/lower/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0114 s (165k iterat
scalar/decode/large/lower/fast-hex-lite/65536
                        time:   [36.399 µs 36.542 µs 36.704 µs]
                        thrpt:  [1.6629 GiB/s 1.6703 GiB/s 1.6768 GiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high severe
scalar/decode/large/lower/hex-crate/65536
                        time:   [169.21 µs 174.97 µs 182.69 µs]
                        thrpt:  [342.10 MiB/s 357.21 MiB/s 369.36 MiB/s]
Benchmarking scalar/decode/large/lower/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.2128 s (11k itera
scalar/decode/large/lower/fast-hex-lite/1048576
                        time:   [580.33 µs 584.89 µs 590.88 µs]
                        thrpt:  [1.6527 GiB/s 1.6696 GiB/s 1.6828 GiB/s]
Found 3 outliers among 30 measurements (10.00%)
  1 (3.33%) low mild
  2 (6.67%) high severe
Benchmarking scalar/decode/large/lower/hex-crate/1048576: Collecting 30 samples in estimated 6.9385 s (1395 iteratio
scalar/decode/large/lower/hex-crate/1048576
                        time:   [4.7717 ms 4.8284 ms 4.9080 ms]
                        thrpt:  [203.75 MiB/s 207.11 MiB/s 209.57 MiB/s]

Benchmarking scalar/decode/large/mixed/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0091 s (165k iterat
scalar/decode/large/mixed/fast-hex-lite/65536
                        time:   [36.266 µs 36.386 µs 36.580 µs]
                        thrpt:  [1.6686 GiB/s 1.6774 GiB/s 1.6830 GiB/s]
Found 2 outliers among 30 measurements (6.67%)
  1 (3.33%) high mild
  1 (3.33%) high severe
scalar/decode/large/mixed/hex-crate/65536
                        time:   [147.60 µs 169.07 µs 188.07 µs]
                        thrpt:  [332.32 MiB/s 369.67 MiB/s 423.44 MiB/s]
Benchmarking scalar/decode/large/mixed/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.1326 s (11k itera
scalar/decode/large/mixed/fast-hex-lite/1048576
                        time:   [569.58 µs 571.26 µs 572.59 µs]
                        thrpt:  [1.7055 GiB/s 1.7095 GiB/s 1.7145 GiB/s]
Found 4 outliers among 30 measurements (13.33%)
  2 (6.67%) low severe
  1 (3.33%) low mild
  1 (3.33%) high severe
Benchmarking scalar/decode/large/mixed/hex-crate/1048576: Collecting 30 samples in estimated 6.8252 s (1395 iteratio
scalar/decode/large/mixed/hex-crate/1048576
                        time:   [4.6462 ms 4.6600 ms 4.6786 ms]
                        thrpt:  [213.74 MiB/s 214.59 MiB/s 215.23 MiB/s]
Found 2 outliers among 30 measurements (6.67%)
  2 (6.67%) high severe

Benchmarking scalar/validate_only/small/lower/validate/32: Collecting 80 samples in estimated 10.000 s (325M iterati
scalar/validate_only/small/lower/validate/32
                        time:   [30.347 ns 30.417 ns 30.502 ns]
                        thrpt:  [1.9541 GiB/s 1.9596 GiB/s 1.9641 GiB/s]
Found 15 outliers among 80 measurements (18.75%)
  5 (6.25%) high mild
  10 (12.50%) high severe
Benchmarking scalar/validate_only/small/lower/validate/256: Collecting 80 samples in estimated 10.001 s (40M iterati
scalar/validate_only/small/lower/validate/256
                        time:   [251.45 ns 259.18 ns 267.80 ns]
                        thrpt:  [1.7806 GiB/s 1.8398 GiB/s 1.8963 GiB/s]

Benchmarking scalar/validate_only/small/mixed/validate/32: Collecting 80 samples in estimated 10.000 s (313M iterati
scalar/validate_only/small/mixed/validate/32
                        time:   [31.141 ns 31.359 ns 31.663 ns]
                        thrpt:  [1.8824 GiB/s 1.9007 GiB/s 1.9140 GiB/s]
Benchmarking scalar/validate_only/small/mixed/validate/256: Collecting 80 samples in estimated 10.001 s (42M iterati
scalar/validate_only/small/mixed/validate/256
                        time:   [245.75 ns 250.72 ns 255.79 ns]
                        thrpt:  [1.8642 GiB/s 1.9019 GiB/s 1.9404 GiB/s]

Benchmarking scalar/validate_only/med/lower/validate/4096: Collecting 60 samples in estimated 8.0066 s (1.5M iterati
scalar/validate_only/med/lower/validate/4096
                        time:   [4.8680 µs 4.8800 µs 4.8968 µs]
                        thrpt:  [1.5580 GiB/s 1.5634 GiB/s 1.5672 GiB/s]
Found 9 outliers among 60 measurements (15.00%)
  8 (13.33%) high mild
  1 (1.67%) high severe

Benchmarking scalar/validate_only/med/mixed/validate/4096: Collecting 60 samples in estimated 8.0085 s (1.5M iterati
scalar/validate_only/med/mixed/validate/4096
                        time:   [4.9745 µs 5.0682 µs 5.1899 µs]
                        thrpt:  [1.4700 GiB/s 1.5053 GiB/s 1.5337 GiB/s]
Found 3 outliers among 60 measurements (5.00%)
  1 (1.67%) high mild
  2 (3.33%) high severe

Benchmarking scalar/validate_only/large/lower/validate/65536: Collecting 30 samples in estimated 6.0315 s (23k itera
scalar/validate_only/large/lower/validate/65536
                        time:   [259.94 µs 262.06 µs 264.11 µs]
                        thrpt:  [473.29 MiB/s 476.98 MiB/s 480.88 MiB/s]
Found 2 outliers among 30 measurements (6.67%)
  2 (6.67%) high mild
Benchmarking scalar/validate_only/large/lower/validate/1048576: Collecting 30 samples in estimated 6.4586 s (1395 it
scalar/validate_only/large/lower/validate/1048576
                        time:   [4.5285 ms 4.5387 ms 4.5530 ms]
                        thrpt:  [439.27 MiB/s 440.66 MiB/s 441.65 MiB/s]
Found 2 outliers among 30 measurements (6.67%)
  1 (3.33%) high mild
  1 (3.33%) high severe

Benchmarking scalar/validate_only/large/mixed/validate/65536: Collecting 30 samples in estimated 6.0475 s (22k itera
scalar/validate_only/large/mixed/validate/65536
                        time:   [257.82 µs 258.24 µs 258.74 µs]
                        thrpt:  [483.10 MiB/s 484.04 MiB/s 484.84 MiB/s]
Found 4 outliers among 30 measurements (13.33%)
  1 (3.33%) high mild
  3 (10.00%) high severe
Benchmarking scalar/validate_only/large/mixed/validate/1048576: Collecting 30 samples in estimated 6.5182 s (1395 it
scalar/validate_only/large/mixed/validate/1048576
                        time:   [4.5265 ms 4.5327 ms 4.5394 ms]
                        thrpt:  [440.59 MiB/s 441.24 MiB/s 441.84 MiB/s]
Found 3 outliers among 30 measurements (10.00%)
  2 (6.67%) high mild
  1 (3.33%) high severe

Benchmarking scalar/encode/small/lower/fast-hex-lite/32: Collecting 80 samples in estimated 10.000 s (736M iteration
scalar/encode/small/lower/fast-hex-lite/32
                        time:   [11.798 ns 11.927 ns 12.121 ns]
                        thrpt:  [2.4586 GiB/s 2.4988 GiB/s 2.5261 GiB/s]
scalar/encode/small/lower/hex-crate/32
                        time:   [14.611 ns 14.655 ns 14.740 ns]
                        thrpt:  [2.0219 GiB/s 2.0336 GiB/s 2.0397 GiB/s]
Found 7 outliers among 80 measurements (8.75%)
  2 (2.50%) high mild
  5 (6.25%) high severe
Benchmarking scalar/encode/small/lower/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (105M iteratio
scalar/encode/small/lower/fast-hex-lite/256
                        time:   [95.170 ns 95.247 ns 95.339 ns]
                        thrpt:  [2.5007 GiB/s 2.5032 GiB/s 2.5052 GiB/s]
Found 10 outliers among 80 measurements (12.50%)
  7 (8.75%) high mild
  3 (3.75%) high severe
scalar/encode/small/lower/hex-crate/256
                        time:   [118.58 ns 118.72 ns 118.91 ns]
                        thrpt:  [2.0050 GiB/s 2.0082 GiB/s 2.0106 GiB/s]
Found 8 outliers among 80 measurements (10.00%)
  3 (3.75%) high mild
  5 (6.25%) high severe

Benchmarking scalar/encode/small/upper/fast-hex-lite/32: Collecting 80 samples in estimated 10.000 s (740M iteration
scalar/encode/small/upper/fast-hex-lite/32
                        time:   [13.490 ns 13.518 ns 13.583 ns]
                        thrpt:  [2.1941 GiB/s 2.2046 GiB/s 2.2091 GiB/s]
Found 9 outliers among 80 measurements (11.25%)
  2 (2.50%) high mild
  7 (8.75%) high severe
Benchmarking scalar/encode/small/upper/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (104M iteratio
scalar/encode/small/upper/fast-hex-lite/256
                        time:   [95.877 ns 96.025 ns 96.196 ns]
                        thrpt:  [2.4785 GiB/s 2.4829 GiB/s 2.4867 GiB/s]
Found 12 outliers among 80 measurements (15.00%)
  6 (7.50%) high mild
  6 (7.50%) high severe

Benchmarking scalar/encode/med/lower/fast-hex-lite/4096: Collecting 60 samples in estimated 8.0007 s (5.4M iteration
scalar/encode/med/lower/fast-hex-lite/4096
                        time:   [1.4622 µs 1.4635 µs 1.4649 µs]
                        thrpt:  [2.6041 GiB/s 2.6066 GiB/s 2.6088 GiB/s]
Found 4 outliers among 60 measurements (6.67%)
  2 (3.33%) high mild
  2 (3.33%) high severe
scalar/encode/med/lower/hex-crate/4096
                        time:   [1.8470 µs 1.8539 µs 1.8631 µs]
                        thrpt:  [2.0475 GiB/s 2.0577 GiB/s 2.0653 GiB/s]
Found 2 outliers among 60 measurements (3.33%)
  1 (1.67%) high mild
  1 (1.67%) high severe

Benchmarking scalar/encode/med/upper/fast-hex-lite/4096: Collecting 60 samples in estimated 8.0004 s (5.4M iteration
scalar/encode/med/upper/fast-hex-lite/4096
                        time:   [1.4700 µs 1.4744 µs 1.4783 µs]
                        thrpt:  [2.5805 GiB/s 2.5873 GiB/s 2.5950 GiB/s]

Benchmarking scalar/encode/large/lower/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0090 s (254k iterat
scalar/encode/large/lower/fast-hex-lite/65536
                        time:   [23.358 µs 23.438 µs 23.562 µs]
                        thrpt:  [2.5904 GiB/s 2.6041 GiB/s 2.6130 GiB/s]
Found 3 outliers among 30 measurements (10.00%)
  2 (6.67%) high mild
  1 (3.33%) high severe
Benchmarking scalar/encode/large/lower/hex-crate/65536: Collecting 30 samples in estimated 6.0004 s (206k iterations
scalar/encode/large/lower/hex-crate/65536
                        time:   [29.131 µs 29.155 µs 29.180 µs]
                        thrpt:  [2.0916 GiB/s 2.0935 GiB/s 2.0952 GiB/s]
Found 2 outliers among 30 measurements (6.67%)
  2 (6.67%) high mild
Benchmarking scalar/encode/large/lower/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.1384 s (16k itera
scalar/encode/large/lower/fast-hex-lite/1048576
                        time:   [376.27 µs 377.60 µs 379.86 µs]
                        thrpt:  [2.5708 GiB/s 2.5863 GiB/s 2.5954 GiB/s]
Found 2 outliers among 30 measurements (6.67%)
  2 (6.67%) high severe
Benchmarking scalar/encode/large/lower/hex-crate/1048576: Collecting 30 samples in estimated 6.1062 s (13k iteration
scalar/encode/large/lower/hex-crate/1048576
                        time:   [467.80 µs 468.30 µs 468.80 µs]
                        thrpt:  [2.0831 GiB/s 2.0854 GiB/s 2.0876 GiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high severe

Benchmarking scalar/encode/large/upper/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0022 s (256k iterat
scalar/encode/large/upper/fast-hex-lite/65536
                        time:   [23.362 µs 23.447 µs 23.593 µs]
                        thrpt:  [2.5870 GiB/s 2.6031 GiB/s 2.6126 GiB/s]
Found 3 outliers among 30 measurements (10.00%)
  1 (3.33%) high mild
  2 (6.67%) high severe
Benchmarking scalar/encode/large/upper/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.1359 s (16k itera
scalar/encode/large/upper/fast-hex-lite/1048576
                        time:   [376.71 µs 377.59 µs 378.32 µs]
                        thrpt:  [2.5813 GiB/s 2.5863 GiB/s 2.5924 GiB/s]
Found 2 outliers among 30 measurements (6.67%)
  2 (6.67%) high mild

Benchmarking scalar/decode_in_place/small/fast-hex-lite/32: Collecting 80 samples in estimated 10.000 s (172M iterat
scalar/decode_in_place/small/fast-hex-lite/32
                        time:   [46.485 ns 46.618 ns 46.772 ns]
                        thrpt:  [652.47 MiB/s 654.63 MiB/s 656.50 MiB/s]
Found 6 outliers among 80 measurements (7.50%)
  4 (5.00%) high mild
  2 (2.50%) high severe
Benchmarking scalar/decode_in_place/small/fast-hex-lite/256: Collecting 80 samples in estimated 10.001 s (27M iterat
scalar/decode_in_place/small/fast-hex-lite/256
                        time:   [339.79 ns 340.56 ns 341.51 ns]
                        thrpt:  [714.89 MiB/s 716.87 MiB/s 718.51 MiB/s]
Found 4 outliers among 80 measurements (5.00%)
  3 (3.75%) high mild
  1 (1.25%) high severe

Benchmarking scalar/decode_in_place/med/fast-hex-lite/4096: Collecting 60 samples in estimated 8.0047 s (1.5M iterat
scalar/decode_in_place/med/fast-hex-lite/4096
                        time:   [5.0870 µs 5.1101 µs 5.1382 µs]
                        thrpt:  [760.24 MiB/s 764.41 MiB/s 767.89 MiB/s]
Found 3 outliers among 60 measurements (5.00%)
  1 (1.67%) high mild
  2 (3.33%) high severe

Benchmarking scalar/decode_in_place/large/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0037 s (73k iter
scalar/decode_in_place/large/fast-hex-lite/65536
                        time:   [80.887 µs 81.674 µs 82.554 µs]
                        thrpt:  [757.08 MiB/s 765.24 MiB/s 772.68 MiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high mild
Benchmarking scalar/decode_in_place/large/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.2405 s (4650 i
scalar/decode_in_place/large/fast-hex-lite/1048576
                        time:   [1.2764 ms 1.2819 ms 1.2899 ms]
                        thrpt:  [775.23 MiB/s 780.10 MiB/s 783.47 MiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high severe

   Compiling fast-hex-lite v0.1.0 (/Users/vlad/Coding/Rust/fast-hex-lite)
    Finished `bench` profile [optimized] target(s) in 13.04s

 Running benches/bench.rs (target/release/deps/bench-53752d2458c2e05d)
Gnuplot not found, using plotters backend
simd/decode/small/lower/fast-hex-lite/32
                        time:   [5.3842 ns 5.4120 ns 5.4512 ns]
                        thrpt:  [5.4671 GiB/s 5.5068 GiB/s 5.5352 GiB/s]
Found 5 outliers among 80 measurements (6.25%)
  4 (5.00%) high mild
  1 (1.25%) high severe
simd/decode/small/lower/hex-crate/32
                        time:   [47.948 ns 48.605 ns 49.297 ns]
                        thrpt:  [619.06 MiB/s 627.87 MiB/s 636.48 MiB/s]
Found 1 outliers among 80 measurements (1.25%)
  1 (1.25%) high mild
Benchmarking simd/decode/small/lower/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (257M iterations
simd/decode/small/lower/fast-hex-lite/256
                        time:   [38.971 ns 39.085 ns 39.249 ns]
                        thrpt:  [6.0746 GiB/s 6.1001 GiB/s 6.1178 GiB/s]
Found 4 outliers among 80 measurements (5.00%)
  2 (2.50%) high mild
  2 (2.50%) high severe
simd/decode/small/lower/hex-crate/256
                        time:   [399.88 ns 401.58 ns 403.45 ns]
                        thrpt:  [605.13 MiB/s 607.96 MiB/s 610.53 MiB/s]
Found 3 outliers among 80 measurements (3.75%)
  2 (2.50%) high mild
  1 (1.25%) high severe

simd/decode/small/mixed/fast-hex-lite/32
                        time:   [5.3904 ns 5.4261 ns 5.4711 ns]
                        thrpt:  [5.4473 GiB/s 5.4924 GiB/s 5.5288 GiB/s]
Found 4 outliers among 80 measurements (5.00%)
  2 (2.50%) high mild
  2 (2.50%) high severe
simd/decode/small/mixed/hex-crate/32
                        time:   [44.091 ns 44.796 ns 45.607 ns]
                        thrpt:  [669.14 MiB/s 681.26 MiB/s 692.15 MiB/s]
Found 1 outliers among 80 measurements (1.25%)
  1 (1.25%) high severe
Benchmarking simd/decode/small/mixed/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (253M iterations
simd/decode/small/mixed/fast-hex-lite/256
                        time:   [39.027 ns 39.156 ns 39.380 ns]
                        thrpt:  [6.0543 GiB/s 6.0890 GiB/s 6.1090 GiB/s]
Found 3 outliers among 80 measurements (3.75%)
  2 (2.50%) high mild
  1 (1.25%) high severe
simd/decode/small/mixed/hex-crate/256
                        time:   [368.19 ns 370.52 ns 372.86 ns]
                        thrpt:  [654.77 MiB/s 658.91 MiB/s 663.08 MiB/s]
Found 1 outliers among 80 measurements (1.25%)
  1 (1.25%) high severe

simd/decode/med/lower/fast-hex-lite/4096
                        time:   [631.66 ns 632.15 ns 632.68 ns]
                        thrpt:  [6.0294 GiB/s 6.0345 GiB/s 6.0392 GiB/s]
Found 6 outliers among 60 measurements (10.00%)
  3 (5.00%) high mild
  3 (5.00%) high severe
simd/decode/med/lower/hex-crate/4096
                        time:   [6.6735 µs 6.6835 µs 6.6952 µs]
                        thrpt:  [583.44 MiB/s 584.47 MiB/s 585.33 MiB/s]
Found 5 outliers among 60 measurements (8.33%)
  1 (1.67%) low severe
  1 (1.67%) low mild
  1 (1.67%) high mild
  2 (3.33%) high severe

simd/decode/med/mixed/fast-hex-lite/4096
                        time:   [631.45 ns 631.93 ns 632.47 ns]
                        thrpt:  [6.0315 GiB/s 6.0366 GiB/s 6.0412 GiB/s]
Found 2 outliers among 60 measurements (3.33%)
  1 (1.67%) high mild
  1 (1.67%) high severe
simd/decode/med/mixed/hex-crate/4096
                        time:   [6.3068 µs 6.3277 µs 6.3621 µs]
                        thrpt:  [613.99 MiB/s 617.32 MiB/s 619.37 MiB/s]
Found 5 outliers among 60 measurements (8.33%)
  1 (1.67%) low severe
  1 (1.67%) high mild
  3 (5.00%) high severe

Benchmarking simd/decode/large/lower/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0022 s (601k iteratio
simd/decode/large/lower/fast-hex-lite/65536
                        time:   [9.9297 µs 9.9408 µs 9.9552 µs]
                        thrpt:  [6.1310 GiB/s 6.1399 GiB/s 6.1467 GiB/s]
simd/decode/large/lower/hex-crate/65536
                        time:   [152.69 µs 160.05 µs 168.21 µs]
                        thrpt:  [371.57 MiB/s 390.49 MiB/s 409.34 MiB/s]
Benchmarking simd/decode/large/lower/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.0369 s (38k iterati
simd/decode/large/lower/fast-hex-lite/1048576
                        time:   [159.76 µs 160.48 µs 161.34 µs]
                        thrpt:  [6.0530 GiB/s 6.0852 GiB/s 6.1125 GiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high mild
Benchmarking simd/decode/large/lower/hex-crate/1048576: Collecting 30 samples in estimated 7.0078 s (1395 iterations
simd/decode/large/lower/hex-crate/1048576
                        time:   [4.9513 ms 4.9876 ms 5.0250 ms]
                        thrpt:  [199.00 MiB/s 200.50 MiB/s 201.97 MiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high mild

Benchmarking simd/decode/large/mixed/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0002 s (599k iteratio
simd/decode/large/mixed/fast-hex-lite/65536
                        time:   [9.9186 µs 9.9277 µs 9.9379 µs]
                        thrpt:  [6.1417 GiB/s 6.1480 GiB/s 6.1536 GiB/s]
Found 4 outliers among 30 measurements (13.33%)
  3 (10.00%) high mild
  1 (3.33%) high severe
simd/decode/large/mixed/hex-crate/65536
                        time:   [146.02 µs 160.02 µs 178.52 µs]
                        thrpt:  [350.10 MiB/s 390.58 MiB/s 428.03 MiB/s]
Benchmarking simd/decode/large/mixed/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.0585 s (38k iterati
simd/decode/large/mixed/fast-hex-lite/1048576
                        time:   [158.35 µs 158.69 µs 159.25 µs]
                        thrpt:  [6.1321 GiB/s 6.1539 GiB/s 6.1672 GiB/s]
Found 3 outliers among 30 measurements (10.00%)
  1 (3.33%) high mild
  2 (6.67%) high severe
Benchmarking simd/decode/large/mixed/hex-crate/1048576: Collecting 30 samples in estimated 6.9375 s (1395 iterations
simd/decode/large/mixed/hex-crate/1048576
                        time:   [4.9376 ms 4.9587 ms 4.9756 ms]
                        thrpt:  [200.98 MiB/s 201.67 MiB/s 202.53 MiB/s]

Benchmarking simd/validate_only/small/lower/validate/32: Collecting 80 samples in estimated 10.000 s (303M iteration
simd/validate_only/small/lower/validate/32
                        time:   [31.992 ns 32.118 ns 32.305 ns]
                        thrpt:  [1.8450 GiB/s 1.8558 GiB/s 1.8631 GiB/s]
Found 2 outliers among 80 measurements (2.50%)
  1 (1.25%) high mild
  1 (1.25%) high severe
Benchmarking simd/validate_only/small/lower/validate/256: Collecting 80 samples in estimated 10.000 s (40M iteration
simd/validate_only/small/lower/validate/256
                        time:   [253.71 ns 259.12 ns 263.90 ns]
                        thrpt:  [1.8069 GiB/s 1.8402 GiB/s 1.8795 GiB/s]
Found 1 outliers among 80 measurements (1.25%)
  1 (1.25%) high mild

Benchmarking simd/validate_only/small/mixed/validate/32: Collecting 80 samples in estimated 10.000 s (307M iteration
simd/validate_only/small/mixed/validate/32
                        time:   [33.184 ns 33.478 ns 33.773 ns]
                        thrpt:  [1.7648 GiB/s 1.7804 GiB/s 1.7962 GiB/s]
Benchmarking simd/validate_only/small/mixed/validate/256: Collecting 80 samples in estimated 10.000 s (38M iteration
simd/validate_only/small/mixed/validate/256
                        time:   [244.77 ns 247.33 ns 250.02 ns]
                        thrpt:  [1.9072 GiB/s 1.9279 GiB/s 1.9481 GiB/s]
Found 1 outliers among 80 measurements (1.25%)
  1 (1.25%) high severe

Benchmarking simd/validate_only/med/lower/validate/4096: Collecting 60 samples in estimated 8.0060 s (1.6M iteration
simd/validate_only/med/lower/validate/4096
                        time:   [4.9042 µs 4.9325 µs 4.9702 µs]
                        thrpt:  [1.5350 GiB/s 1.5468 GiB/s 1.5557 GiB/s]
Found 12 outliers among 60 measurements (20.00%)
  2 (3.33%) high mild
  10 (16.67%) high severe

Benchmarking simd/validate_only/med/mixed/validate/4096: Collecting 60 samples in estimated 8.0063 s (1.5M iteration
simd/validate_only/med/mixed/validate/4096
                        time:   [5.0910 µs 5.2434 µs 5.4418 µs]
                        thrpt:  [1.4020 GiB/s 1.4550 GiB/s 1.4986 GiB/s]
Found 18 outliers among 60 measurements (30.00%)
  13 (21.67%) low severe
  1 (1.67%) high mild
  4 (6.67%) high severe

Benchmarking simd/validate_only/large/lower/validate/65536: Collecting 30 samples in estimated 6.0381 s (22k iterati
simd/validate_only/large/lower/validate/65536
                        time:   [262.40 µs 265.06 µs 268.66 µs]
                        thrpt:  [465.27 MiB/s 471.59 MiB/s 476.37 MiB/s]
Found 7 outliers among 30 measurements (23.33%)
  7 (23.33%) low severe
Benchmarking simd/validate_only/large/lower/validate/1048576: Collecting 30 samples in estimated 6.6511 s (1395 iter
simd/validate_only/large/lower/validate/1048576
                        time:   [4.5286 ms 4.5371 ms 4.5455 ms]
                        thrpt:  [440.00 MiB/s 440.81 MiB/s 441.64 MiB/s]
Found 2 outliers among 30 measurements (6.67%)
  2 (6.67%) high severe

Benchmarking simd/validate_only/large/mixed/validate/65536: Collecting 30 samples in estimated 6.0599 s (23k iterati
simd/validate_only/large/mixed/validate/65536
                        time:   [257.19 µs 257.80 µs 258.44 µs]
                        thrpt:  [483.67 MiB/s 484.86 MiB/s 486.02 MiB/s]
Found 2 outliers among 30 measurements (6.67%)
  2 (6.67%) high mild
Benchmarking simd/validate_only/large/mixed/validate/1048576: Collecting 30 samples in estimated 6.6019 s (1395 iter
simd/validate_only/large/mixed/validate/1048576
                        time:   [4.5470 ms 4.5691 ms 4.6047 ms]
                        thrpt:  [434.34 MiB/s 437.73 MiB/s 439.85 MiB/s]

simd/encode/small/lower/fast-hex-lite/32
                        time:   [12.290 ns 12.636 ns 12.960 ns]
                        thrpt:  [2.2995 GiB/s 2.3586 GiB/s 2.4249 GiB/s]
simd/encode/small/lower/hex-crate/32
                        time:   [13.729 ns 13.744 ns 13.763 ns]
                        thrpt:  [2.1653 GiB/s 2.1684 GiB/s 2.1708 GiB/s]
Found 9 outliers among 80 measurements (11.25%)
  5 (6.25%) high mild
  4 (5.00%) high severe
Benchmarking simd/encode/small/lower/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (104M iterations
simd/encode/small/lower/fast-hex-lite/256
                        time:   [95.559 ns 95.706 ns 95.983 ns]
                        thrpt:  [2.4840 GiB/s 2.4911 GiB/s 2.4950 GiB/s]
Found 13 outliers among 80 measurements (16.25%)
  1 (1.25%) low mild
  5 (6.25%) high mild
  7 (8.75%) high severe
simd/encode/small/lower/hex-crate/256
                        time:   [115.02 ns 116.03 ns 117.18 ns]
                        thrpt:  [2.0347 GiB/s 2.0549 GiB/s 2.0728 GiB/s]
Found 4 outliers among 80 measurements (5.00%)
  4 (5.00%) high mild

simd/encode/small/upper/fast-hex-lite/32
                        time:   [12.235 ns 12.547 ns 12.939 ns]
                        thrpt:  [2.3033 GiB/s 2.3753 GiB/s 2.4358 GiB/s]
Found 19 outliers among 80 measurements (23.75%)
  19 (23.75%) low severe
Benchmarking simd/encode/small/upper/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (104M iterations
simd/encode/small/upper/fast-hex-lite/256
                        time:   [95.777 ns 95.966 ns 96.380 ns]
                        thrpt:  [2.4737 GiB/s 2.4844 GiB/s 2.4893 GiB/s]
Found 8 outliers among 80 measurements (10.00%)
  5 (6.25%) high mild
  3 (3.75%) high severe

simd/encode/med/lower/fast-hex-lite/4096
                        time:   [1.4638 µs 1.4648 µs 1.4659 µs]
                        thrpt:  [2.6022 GiB/s 2.6042 GiB/s 2.6060 GiB/s]
Found 4 outliers among 60 measurements (6.67%)
  3 (5.00%) high mild
  1 (1.67%) high severe
simd/encode/med/lower/hex-crate/4096
                        time:   [1.7136 µs 1.7148 µs 1.7162 µs]
                        thrpt:  [2.2228 GiB/s 2.2246 GiB/s 2.2262 GiB/s]
Found 6 outliers among 60 measurements (10.00%)
  3 (5.00%) high mild
  3 (5.00%) high severe

simd/encode/med/upper/fast-hex-lite/4096
                        time:   [1.4639 µs 1.4657 µs 1.4677 µs]
                        thrpt:  [2.5990 GiB/s 2.6026 GiB/s 2.6058 GiB/s]
Found 6 outliers among 60 measurements (10.00%)
  5 (8.33%) high mild
  1 (1.67%) high severe

Benchmarking simd/encode/large/lower/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0033 s (253k iteratio
simd/encode/large/lower/fast-hex-lite/65536
                        time:   [23.390 µs 23.427 µs 23.461 µs]
                        thrpt:  [2.6016 GiB/s 2.6053 GiB/s 2.6095 GiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high severe
simd/encode/large/lower/hex-crate/65536
                        time:   [27.458 µs 27.582 µs 27.734 µs]
                        thrpt:  [2.2007 GiB/s 2.2128 GiB/s 2.2228 GiB/s]
Found 4 outliers among 30 measurements (13.33%)
  4 (13.33%) high mild
Benchmarking simd/encode/large/lower/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.0892 s (16k iterati
simd/encode/large/lower/fast-hex-lite/1048576
                        time:   [376.53 µs 377.38 µs 378.44 µs]
                        thrpt:  [2.5805 GiB/s 2.5878 GiB/s 2.5936 GiB/s]
simd/encode/large/lower/hex-crate/1048576
                        time:   [439.61 µs 440.13 µs 440.60 µs]
                        thrpt:  [2.2165 GiB/s 2.2188 GiB/s 2.2214 GiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high mild

Benchmarking simd/encode/large/upper/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0012 s (252k iteratio
simd/encode/large/upper/fast-hex-lite/65536
                        time:   [23.499 µs 23.621 µs 23.788 µs]
                        thrpt:  [2.5658 GiB/s 2.5839 GiB/s 2.5973 GiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high severe
Benchmarking simd/encode/large/upper/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.0395 s (16k iterati
simd/encode/large/upper/fast-hex-lite/1048576
                        time:   [380.92 µs 382.67 µs 384.49 µs]
                        thrpt:  [2.5399 GiB/s 2.5520 GiB/s 2.5637 GiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high mild

Benchmarking simd/decode_in_place/small/fast-hex-lite/32: Collecting 80 samples in estimated 10.000 s (168M iteratio
simd/decode_in_place/small/fast-hex-lite/32
                        time:   [46.673 ns 46.961 ns 47.271 ns]
                        thrpt:  [645.59 MiB/s 649.86 MiB/s 653.86 MiB/s]
Found 1 outliers among 80 measurements (1.25%)
  1 (1.25%) high severe
Benchmarking simd/decode_in_place/small/fast-hex-lite/256: Collecting 80 samples in estimated 10.000 s (27M iteratio
simd/decode_in_place/small/fast-hex-lite/256
                        time:   [342.81 ns 344.24 ns 347.01 ns]
                        thrpt:  [703.56 MiB/s 709.21 MiB/s 712.18 MiB/s]
Found 5 outliers among 80 measurements (6.25%)
  4 (5.00%) high mild
  1 (1.25%) high severe

Benchmarking simd/decode_in_place/med/fast-hex-lite/4096: Collecting 60 samples in estimated 8.0039 s (1.5M iteratio
simd/decode_in_place/med/fast-hex-lite/4096
                        time:   [5.0269 µs 5.0375 µs 5.0479 µs]
                        thrpt:  [773.84 MiB/s 775.44 MiB/s 777.07 MiB/s]
Found 1 outliers among 60 measurements (1.67%)
  1 (1.67%) high mild

Benchmarking simd/decode_in_place/large/fast-hex-lite/65536: Collecting 30 samples in estimated 6.0129 s (73k iterat
simd/decode_in_place/large/fast-hex-lite/65536
                        time:   [80.563 µs 81.131 µs 81.779 µs]
                        thrpt:  [764.26 MiB/s 770.36 MiB/s 775.79 MiB/s]
Found 2 outliers among 30 measurements (6.67%)
  2 (6.67%) high mild
Benchmarking simd/decode_in_place/large/fast-hex-lite/1048576: Collecting 30 samples in estimated 6.0785 s (4650 ite
simd/decode_in_place/large/fast-hex-lite/1048576
                        time:   [1.2723 ms 1.2744 ms 1.2774 ms]
                        thrpt:  [782.84 MiB/s 784.69 MiB/s 786.00 MiB/s]
Found 1 outliers among 30 measurements (3.33%)
  1 (3.33%) high severe