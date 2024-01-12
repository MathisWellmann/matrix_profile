//! This module contains the naive implementation using convolutions with an O(n * m) runtime.

use simd_euclidean::Vectorized;

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
/// `dim`: The dimensionality of the time-series.
///        The datapoints of dimensions are expected to be right next to each other in the slice.
///        E.g For a 2 dimensional timeseries with x0, x1, x2, x3 as the first dimension and
///        y0, y1, y2, y3 as the second dimension, the datapoints should appear as such in the slices:
///        x0, y0, x1, y1, x2, y2, x3, y3.
///
pub fn distance_profile(history: &[f32], window: &[f32], dim: usize, normalize: bool) -> Vec<f32> {
    match normalize {
        false => distance_profile_raw(history, window, dim),
        true => distance_profile_normalized(history, window, dim),
    }
}

fn distance_profile_normalized(history: &[f32], window: &[f32], dim: usize) -> Vec<f32> {
    assert!(dim > 0, "A time series must be at least 1 dimensional");
    let normalized_window = normalize(window);
    Vec::from_iter((0..history.len() - window.len() + 1).step_by(dim).map(|i| {
        let comp = normalize(&history[i..i + window.len()]);
        Vectorized::squared_distance(&normalized_window, &comp)
    }))
}

fn distance_profile_raw(history: &[f32], window: &[f32], dim: usize) -> Vec<f32> {
    assert!(dim > 0, "A time series must be at least 1 dimensional");
    Vec::from_iter((0..history.len() - window.len() + 1).step_by(dim).map(|i| {
        let comp = &history[i..i + window.len()];
        Vectorized::squared_distance(window, comp)
    }))
}

/// Perform a high-low normalization
fn normalize(vals: &[f32]) -> Vec<f32> {
    let mut high = vals[0];
    let mut low = vals[0];

    for v in vals.iter().skip(1) {
        if *v < low {
            low = *v;
        }
        if *v > high {
            high = *v;
        }
    }

    Vec::from_iter(vals.iter().map(|v| scale(low, high, *v)))
}

/// Scaling a `value` from range (`from_min`..`from_max`) to (0..1)
#[inline]
fn scale(from_min: f32, from_max: f32, value: f32) -> f32 {
    (value - from_min) / (from_max - from_min)
}

/// Find all starting indices of the sequence which has the lowest euclidean distance to the specified `window`.
/// The `window` is assumed to be non-overlapping with `history` and located at the end as such:
/// | `history` | `window` |
/// This ensures the trivial match of `window` == `window` is not returned nor any indices that are too close.
pub fn index_of_motif_iterator(
    history: &[f32],
    window: &[f32],
    dim: usize,
    normalize: bool,
) -> impl Iterator<Item = usize> {
    let profile = distance_profile(history, window, dim, normalize);
    let mut profile_with_index = Vec::from_iter(profile.into_iter().enumerate());
    profile_with_index.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    profile_with_index.into_iter().map(|(i, _distance)| i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused)]
    use criterion; // Used in benchmarks.

    #[test]
    fn test_distance_profile_raw() {
        let history = Vec::<f32>::from_iter((0..10).map(|v| v as f32));
        let window = &history[8..];
        let history = &history[0..8];
        println!("history: {history:?}");
        println!("window: {window:?}");

        let profile = distance_profile(history, window, 1, false);
        println!("profile: {profile:?}");
        assert_eq!(&profile, &[128.0, 98.0, 72.0, 50.0, 32.0, 18.0, 8.0]);
    }

    #[test]
    fn test_scale() {
        assert_eq!(scale(0.0, 1.0, 0.0), 0.0);
        assert_eq!(scale(0.0, 1.0, 1.0), 1.0);
        assert_eq!(scale(0.0, 2.0, 1.0), 0.5);
        assert_eq!(scale(1.0, 2.0, 1.0), 0.0);
        assert_eq!(scale(1.0, 2.0, 2.0), 1.0);
    }

    #[test]
    fn test_normalize() {
        let vals = Vec::from_iter((0..10).map(|v| v as f32));
        assert_eq!(
            normalize(&vals),
            Vec::from_iter((0..10).map(|v| (v as f32) / 9.0))
        )
    }

    #[test]
    fn test_distance_profile_2d() {
        let xs = Vec::<f32>::from_iter((0..10).map(|v| v as f32));
        let ys = Vec::<f32>::from_iter((0..10).map(|v| v as f32));
        let history = Vec::from_iter(xs.into_iter().zip(ys).map(|(x, y)| vec![x, y]).flatten());
        println!("2D history: {history:?}");

        let window = &history[16..];
        let history = &history[..16];
        println!("history: {history:?}");
        println!("window: {window:?}");

        let profile = distance_profile(history, window, 2, false);
        println!("profile: {profile:?}");
        assert_eq!(&profile, &[256.0, 196.0, 144.0, 100.0, 64.0, 36.0, 16.0]);
    }

    #[test]
    fn test_index_of_motif_iterator() {
        let history = Vec::from_iter((0..10).map(|v| v as f32));
        let window = Vec::from_iter((10..13).map(|v| v as f32));
        assert_eq!(
            Vec::from_iter(index_of_motif_iterator(&history, &window, 1, false)),
            vec![7, 6, 5, 4, 3, 2, 1, 0]
        );
    }
}
