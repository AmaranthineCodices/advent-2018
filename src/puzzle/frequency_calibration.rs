use regex::Regex;

use ::Puzzle;

lazy_static! {
    static ref OFFSET_REGEX: Regex = Regex::new(r"([+-]\d+)").unwrap();
}

pub struct FrequencyCalibration;

impl Puzzle for FrequencyCalibration {
    fn solve(&self, input: &str) -> String {
        let mut correction = 0;

        for capture in OFFSET_REGEX.captures_iter(input) {
            let offset_string = capture.get(1).unwrap().as_str();
            let offset_number = offset_string.parse::<isize>().unwrap();
            correction += offset_number;
        }

        correction.to_string()
    }
}