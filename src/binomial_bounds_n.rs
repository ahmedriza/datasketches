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
}
