mod frequency_calibration;
mod inventory_management;

pub use self::frequency_calibration::{FrequencyCalibration, FrequencyDuplication};
pub use self::inventory_management::{ChecksumCalculation, CommonInventoryChars};

pub trait Puzzle {
    fn solve(&self, input: &str) -> String;
}
