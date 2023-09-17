///
/// Enables the estimation of error bounds given a sample set size, the sampling
/// probability theta, the number of standard deviations and a simple no_data_seen flag.  This can
/// be used to estimate error bounds for fixed threshold sampling as well as the error bounds
/// calculations for sketches.
///
/// See:
///
/// * [datasketches-java](https://github.com/apache/datasketches-java)
/// * [datasketches-cpp](https://github.com/apache/datasketches-cpp)
///
#[derive(Default)]
pub struct BinomialBoundsN {
    pub delta_of_num_sdev: Vec<f64>,
}

#[allow(clippy::excessive_precision)]
impl BinomialBoundsN {
    pub fn new() -> Self {
        let delta_of_num_sdev = vec![
            0.5000000000000000000, // = 0.5 (1 + erf(0)
            0.1586553191586026479, // = 0.5 (1 + erf((-1/sqrt(2))))
            0.0227502618904135701, // = 0.5 (1 + erf((-2/sqrt(2))))
            0.0013498126861731796, // = 0.5 (1 + erf((-3/sqrt(2))))
        ];
        BinomialBoundsN { delta_of_num_sdev }
    }

    // our "classic" lower bound, but now with continuity correction
    #[allow(unused)]
    fn cont_classic_lb(&self, num_samples_f: f64, theta: f64, num_sdev: f64) -> f64 {
        let n_hat = (num_samples_f - 0.5) / theta;
        let b = num_sdev * ((1.0 - theta) / theta).sqrt();
        let d = 0.5 * b * ((b * b) + (4.0 * n_hat)).sqrt();
        let centre = n_hat + (0.5 * (b * b));
        centre - d
    }

    #[allow(unused)]
    // our "classic" upper bound, but now with continuity correction
    fn cont_classic_ub(&self, num_samples_f: f64, theta: f64, num_sdev: f64) -> f64 {
        let n_hat = (num_samples_f + 0.5) / theta;
        let b = num_sdev * ((1.0 - theta) / theta).sqrt();
        let d = 0.5 * b * ((b * b) + (4.0 * n_hat)).sqrt();
        let centre = n_hat + (0.5 * (b * b));
        centre + d
    }
}

// -------------------------------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::BinomialBoundsN;

    #[allow(unused)]
    #[test]
    fn test_cont_classic_lb() {
        let bounds = BinomialBoundsN::new();
        let num_samples_f = 500.0;
        let theta = 0.001;
        let num_sdev = 2.0;

        let lb = bounds.cont_classic_lb(num_samples_f, theta, num_sdev);
        let ub = bounds.cont_classic_ub(num_samples_f, theta, num_sdev);
        println!("bounds: [{},{}]", lb, ub);
    }
}
