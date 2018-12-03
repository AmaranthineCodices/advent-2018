mod frequency_calibration;

pub use self::frequency_calibration::{ FrequencyCalibration, FrequencyDuplication };

pub trait Puzzle {
    fn solve(&self, input: &str) -> String;
}