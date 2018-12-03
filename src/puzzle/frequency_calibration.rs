use regex::Regex;

use Puzzle;

lazy_static! {
    static ref OFFSET_REGEX: Regex = Regex::new(r"([+-]\d+)").unwrap();
}

pub struct FrequencyCalibration;

impl Puzzle for FrequencyCalibration {
    fn solve(&self, input: &str) -> String {
        OFFSET_REGEX
            .captures_iter(input)
            .map(|c| c.get(1).unwrap().as_str().parse::<isize>().unwrap())
            .sum::<isize>()
            .to_string()
    }
}

pub struct FrequencyDuplication;

impl Puzzle for FrequencyDuplication {
    fn solve(&self, input: &str) -> String {
        let offsets: Vec<isize> = OFFSET_REGEX
            .captures_iter(input)
            .map(|c| c.get(1).unwrap().as_str().parse::<isize>().unwrap())
            .collect();
        let final_frequency: isize = offsets.iter().sum();
        let seen_frequencies: Vec<isize> = {
            let mut current_frequency = 0;
            let mut result = Vec::with_capacity(offsets.len());

            for offset in &offsets {
                current_frequency += offset;
                result.push(current_frequency);
            }

            result
        };

        let mut current_frequency = final_frequency;

        loop {
            for offset in &offsets {
                current_frequency += offset;

                for seen_frequency in &seen_frequencies {
                    if current_frequency == *seen_frequency {
                        return current_frequency.to_string();
                    }
                }
            }
        }
    }
}
