mod frequency_calibration;

pub use self::frequency_calibration::FrequencyCalibration;

pub trait Puzzle {
    fn solve(&self, input: &str) -> String;
}