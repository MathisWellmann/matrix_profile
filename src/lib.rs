#![feature(portable_simd)]
#![deny(unused_imports, unused_crate_dependencies, missing_docs)]

//! Crate for computing the matrix profile of a timeseries.

use std::simd::f32x16;

use simd_euclidean::Vectorized;

#[cfg(test)]
mod load_from_csv;

/// Compute the squared euclidean distance
/// TODO: make generic over SIMD width.
pub fn squared_euclidean_distance_simd(a: f32x16, b: f32x16) -> f32 {
    let diff = a - b;
    let squared = diff * diff;

    squared.as_array().iter().sum()
}

/// To compare the performance of SIMD against slice variant.
pub fn squared_euclidean_distance_slice(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(a, b)| (a - b).powi(2)).sum()
}

/// A non-simd version of the distance profile.
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
        let dist = squared_euclidean_distance_slice(window, comp);
        out[i + window.len() - 1] = dist;
    }
    out
}

/// A simd version of the distance profile.
///
/// # Arguments:
/// `window`: The most recent sliding window which to compute a vector of euclidian distances agains.
/// `history`: All the previous datapoints, including the `window` or not.
///
/// # Returns:
/// A vector of length `history.len()`.
pub fn distance_profile_simd(window: &[f32], history: &[f32]) -> Vec<f32> {
    const WINDOW_SIZE: usize = 64;
    debug_assert_eq!(WINDOW_SIZE, window.len());
    let mut out = vec![0.0; history.len()];
    for i in 0..history.len() - WINDOW_SIZE {
        let comp = &history[i..i + WINDOW_SIZE];
        let dist = Vectorized::distance(window, comp);
        out[i + WINDOW_SIZE - 1] = dist;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    use criterion; // Used in benchmarks.

    #[test]
    fn squared_euclidean_distance_test() {
        let a = f32x16::from_array([2.0; 16]);
        let b = f32x16::from_array([3.0; 16]);
        println!("a: {a:?}");
        println!("b: {b:?}");
        let dist = squared_euclidean_distance_simd(a, b);
        println!("dist: {dist}");
        assert_eq!(dist, 16.0);
    }

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
