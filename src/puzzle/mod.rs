mod fabric_slicing;
mod frequency_calibration;
mod inventory_management;

pub use self::fabric_slicing::OverlappingFabricClaims;
pub use self::frequency_calibration::{FrequencyCalibration, FrequencyDuplication};
pub use self::inventory_management::{ChecksumCalculation, CommonInventoryChars};

pub trait Puzzle {
    fn solve(&self, input: &str) -> String;
}
