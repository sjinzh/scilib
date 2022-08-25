//!
//! # Basic math functions for series
//! 
//! This module provides access to many useful function that are not provided by the base Rust.
//! 

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// # Finds the maximum value in a slice
/// 
/// ## Definition
/// Looks for the largest number in the given set.
/// 
/// ## Inputs
/// - `val`: the slice with the numbers to compare
/// 
/// Returns the largest in the set.
/// 
/// ## Example
/// ```
/// # use scilib::math::series::max_slice;
/// let v: Vec<f64> = vec![0.0, 1.2, -0.1, 5.2, 0.254, 2.8];
/// let m: f64 = max_slice(&v);
/// assert_eq!(m, 5.2);
/// ```
pub fn max_slice<T>(val: &[T]) -> T 
where T: std::cmp::PartialOrd + Copy {

    let mut current_max: &T = &val[0];

    for v in val.iter() {
        if v > current_max {
            current_max = v;
        }
    }

    *current_max
}

/// # Finds the minimum value in a slice
/// 
/// ## Definition
/// Looks for the smallest number in the given set.
/// 
/// ## Inputs
/// - `val`: the slice with the numbers to compare
/// 
/// Returns the smallest in the set.
/// 
/// ## Example
/// ```
/// # use scilib::math::series::min_slice;
/// let v: Vec<f64> = vec![0.0, 1.2, -0.1, 5.2, 0.254, 2.8];
/// let m: f64 = min_slice(&v);
/// assert_eq!(m, -0.1);
/// ```
pub fn min_slice<T>(val: &[T]) -> T 
where T: std::cmp::PartialOrd + Copy {

    let mut current_min: &T = &val[0];

    for v in val.iter() {
        if v < current_min {
            current_min = v;
        }
    }

    *current_min
}

/// # Mean value of a series
/// 
/// ## Definition
/// We follow the mathematical definition of the mean:
/// $$
/// m = \frac{1}{n} \sum^{n}_{i = 1} x_i
/// $$
/// 
/// ## Inputs
/// - `val`: the slice of the series to compute
/// 
/// Returns the mean value of the series.
/// 
/// ## Example
/// ```
/// # use scilib::math::basic::mean;
/// # use scilib::range;
/// let x: Vec<f64> = range::linear(0, 5, 6);
/// let m: f64 = mean(&x);
/// assert_eq!(m, 2.5);
/// ```
pub fn mean(val: &[f64]) -> f64 {

    val.iter().fold(0.0, |sum, v| sum + v) / val.len() as f64
}

/// # Standard deviation of a series
/// 
/// ## Definition
/// As for the mean we follow the mathematical definition of the standard deviation:
/// $$
/// \sigma = \sqrt{ \frac{1}{n} \sum^{n}_{i = 1} (x_i - m)^2 }
/// $$
/// Where $m$ is the mean of the series.
/// 
/// ## Inputs
/// - `val`: the slice of the series to compute
/// 
/// Returns the standard deviation value of the series.
/// 
/// ## Example
/// ```
/// # use scilib::math::basic::std_dev;
/// # use scilib::range;
/// let x: Vec<f64> = range::linear(0, 5, 6);
/// let s: f64 = std_dev(&x);
/// assert!((s - 1.707825127659933).abs() < 1e-10);
/// ```
pub fn std_dev(val: &[f64]) -> f64 {

    let mean: f64 = mean(val);
    (val.iter().fold(0.0, |sum, v| sum + (v - mean).powi(2)) / val.len() as f64).sqrt()
}

/// # Min-Max scaling of a series
/// 
/// ## Definition
/// Min-Max scaling compresses all the data points passed in a series between two arbitrary values a and b.
/// $$
/// x_{s} = \frac{a + (x - min(x))(b - a)}{max(x) - min(x)}
/// $$
/// 
/// ## Inputs
/// - `val`: the series to scale
/// - `a`: the minimum to scale to
/// - `b`: the maximum to scale to
/// 
/// Returns the new series between a and b.
/// 
/// ## Example
/// ```
/// # use scilib::range;
/// # use scilib::math::series::scale_min_max;
/// let x: Vec<f64> = range::linear(1, 6, 7);
/// let n: Vec<f64> = scale_min_max(&x, 2.0, -1.0);
/// assert_eq!(n[0], 2.0);
/// assert_eq!(n[3], 0.5);
/// assert_eq!(n[6], -1.0);
/// ```
pub fn scale_min_max(val: &[f64], a: f64, b: f64) -> Vec<f64> {

    let max_val: f64 = max_slice(&val);     // We find the max of the slice
    let min_val: f64 = min_slice(&val);     // We find the min of the slice
    let ba: f64 = b - a;                    // We compute the difference for the top
    let div: f64 = max_val - min_val;       // We compute the difference for the divisor

    val.iter().map(|x| {                    // We go through each value to scale
        a + (x - min_val) * ba / div        // Scaling
    }).collect()                            // Returning the right type of vector
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
