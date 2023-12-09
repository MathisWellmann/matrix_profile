#![deny(unused_imports, unused_crate_dependencies, missing_docs)]

//! Crate for computing the matrix profile of a timeseries.

use simd_euclidean::Vectorized;

#[cfg(test)]
mod load_from_csv;

/// A simd version of the distance profile.
///
/// # Arguments:
/// `window`: The most recent sliding window which to compute a vector of euclidian distances agains.
/// `history`: All the previous datapoints, including the `window` or not.
///
/// # Returns:
/// A vector of length `history.len()`.
pub fn distance_profile(window: &[f32], history: &[f32]) -> Vec<f32> {
    let mut out = vec![0.0; history.len()];
    for i in 0..history.len() - window.len() {
        let comp = &history[i..i + window.len()];
        let dist = Vectorized::distance(window, comp);
        out[i + window.len() - 1] = dist;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    use criterion; // Used in benchmarks.

    #[test]
    fn test_distance_profile() {
        let a = Vec::<f32>::from_iter((0..10).map(|v| v as f32));
        let window = &a[8..];
        println!("a: {a:?}");
        println!("window: {window:?}");

        let profile = distance_profile(window, &a);
        println!("profile: {profile:?}");
        assert_eq!(profile.len(), a.len());
        assert_eq!(profile[9], 0.0);
    }
}
