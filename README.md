# Distance Profile / Matrix Profile
TLDR: The `distance_profile` computes a euclidean distance from a sliding window of `history` to a reference `window`.

For a deep dive check out the [UCR](https://www.cs.ucr.edu/~eamonn/MatrixProfile.html) page on the topic.

Note that `RUSTFLAGS="-C targe-cpu=native"` shows performance improvements of up to 25% for larger window sizes.
This is because we leverage `SIMD` instructions when possible.
Run `cargo bench` with and without the flag to see for yourself.
