# Distance Profile / Matrix Profile
TLDR: The `distance_profile` computes a euclidean distance of a sliding window to a reference `window`.

For a deep dive check out the [UCR](https://www.cs.ucr.edu/~eamonn/MatrixProfile.html) page on the topic.

Note that `RUSTFLAGS="-C targe-cpu=native"` shows performance improvements of up to 25% for larger window sizes.
This is because we leverage `SIMD` instructions when possible.
Run `cargo bench` with and without the flag to see for yourself.

### Performance
This implementation aims to be as fast as possible,
however the obvious truth is that the runtime scales in `O(n * m)` where `n` is the number of datapoints
in the timeseries `history` and `m` is the `window` length.

But don't worry, benchmarks are ripping fast (on an Intel 13900KS with `RUSTFLAGS="-C target-cpu=native"`) (2023-12-06 @ `facbaf6`):

| HistoryLen | WindowLen | Runtime     |
|------------|-----------|-------------|
| 100_000    |   16      |  272 micros |
| 100_000    |   32      |  468 micros |
| 100_000    |   64      |  446 micros |
| 100_000    |  128      |  822 micros |
| 100_000    |  256      |  1.4986 ms  |
| 100_000    |  512      |  2.7689 ms  |
| 100_000    | 1024      |  5.4858 ms  |
| 250_000    |   16      |  676 micros |
| 250_000    |   32      |  1.1709 ms  |
| 250_000    |   64      |  1.1121 ms  |
| 250_000    |  128      |  1.9726 ms  |
| 250_000    |  256      |  3.6428 ms  |
| 250_000    |  512      |  7.0411 ms  |
| 250_000    | 1024      | 13.937  ms  |
| 500_000    |   16      |  1.3901 ms  |
| 500_000    |   32      |  2.3447 ms  |
| 500_000    |   64      |  2.2555 ms  |
| 500_000    |  128      |  3.9904 ms  |
| 500_000    |  256      |  7.3124 ms  |
| 500_000    |  512      | 14.150  ms  |
| 500_000    | 1024      | 27.290  ms  |
