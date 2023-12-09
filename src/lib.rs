#![feature(portable_simd)]
#![deny(unused_imports, unused_crate_dependencies, missing_docs)]

//! Crate for computing the matrix profile of a timeseries.

use std::simd::f32x16;

#[cfg(test)]
mod load_from_csv;

/// Compute the squared euclidean distance
fn squared_euclidean_distance(a: f32x16, b: f32x16) {
    let diff = a - b;
    println!("diff: {diff:?}");

    let squared = diff * diff;
    println!("squared: {squared:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_profil_on_trade_data() {
        // let prices = load_from_csv::load_prices_from_csv("./data/Bitmex_XBTUSD_1M.csv");
        // debug_assert_eq!(prices.len(), 1_000_000);

        let a = f32x16::from_array([2.0; 16]);
        let b = f32x16::from_array([3.0; 16]);
        println!("a: {a:?}");
        println!("b: {b:?}");
        squared_euclidean_distance(a, b);
    }
}
