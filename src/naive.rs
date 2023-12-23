//! This module contains the naive implementation using convolutions with an O(n * m) runtime.

use simd_euclidean::Vectorized;
use tracing::debug;

/// The `distance_profile` computes a euclidean distance from a sliding window of `history` to a reference `window`.
///
/// Note that the timeseries data should be stationary / be in the same value range.
/// Otherwise you may bias the distance computation.
/// This uses the squared euclidean distance.
///
/// The `window` is assumed to be non-overlapping with `history` and located at the end as such:
/// | `history` | `window` |
/// This ensures the trivial match of `window` == `window` is not returned nor any indices that are too close.
///
/// # Arguments:
/// `window`: The most recent sliding window which to compute a vector of euclidian distances agains.
/// `history`: All the previous datapoints, including the `window` or not.
///
pub fn distance_profile(history: &[f32], window: &[f32]) -> Vec<f32> {
    Vec::from_iter((0..history.len() - window.len() + 1).map(|i| {
        let comp = &history[i..i + window.len()];
        Vectorized::squared_distance(window, comp)
    }))
}

/// Find the starting index of the sequence which has the lowest euclidean distance to the specified `window`.
/// The `window` is assumed to be non-overlapping with `history` and located at the end as such:
/// | `history` | `window` |
/// This ensures the trivial match of `window` == `window` is not returned nor any indices that are too close.
pub fn index_of_motif(history: &[f32], window: &[f32]) -> Option<usize> {
    let profile = distance_profile(history, window);

    let min_idx = profile
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.total_cmp(b))
        .map(|(index, _)| index)?;
    debug!("min_idx: {min_idx}");

    Some(min_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    use criterion; // Used in benchmarks.

    #[test]
    fn test_distance_profile() {
        let history = Vec::<f32>::from_iter((0..10).map(|v| v as f32));
        let window = &history[8..];
        let history = &history[0..8];
        println!("history: {history:?}");
        println!("window: {window:?}");

        let profile = distance_profile(history, window);
        println!("profile: {profile:?}");
        assert_eq!(&profile, &[128.0, 98.0, 72.0, 50.0, 32.0, 18.0, 8.0]);
    }

    #[test]
    fn test_index_with_most_similar_sequence() {
        let history = Vec::<f32>::from_iter((0..10).map(|v| v as f32));
        let window = &history[8..];
        let history = &history[0..8];
        println!("history: {history:?}");
        println!("window: {window:?}");

        let idx = index_of_motif(history, window).expect("Is Some");
        println!("idx: {idx}");
        assert_eq!(idx, 6);
    }
}
