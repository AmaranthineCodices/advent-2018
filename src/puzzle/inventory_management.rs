use std::str;

use Puzzle;

const CHARS: &str = "abcdefghijklmnopqrstuvwxyz";

fn has_n_chars(string: &str, n: usize) -> bool {
    for test_char in CHARS.chars() {
        let count = string.chars().filter(|c| *c == test_char).count();
        if count == n {
            return true;
        }
    }

    false
}

pub struct ChecksumCalculation;

impl Puzzle for ChecksumCalculation {
    fn solve(&self, input: &str) -> String {
        let count_with_two = input.lines().filter(|s| has_n_chars(s, 2)).count();
        let count_with_three = input.lines().filter(|s| has_n_chars(s, 3)).count();
        (count_with_two * count_with_three).to_string()
    }
}

// Returns a string containing the shared ASCII characters, in order of appearance, of two &strs
fn common_chars(string1: &str, string2: &str) -> String {
    let mut result: Vec<u8> = vec![];
    let bytes1 = string1.as_bytes();
    let bytes2 = string2.as_bytes();

    for i in 0..string1.len() {
        if bytes1[i] == bytes2[i] {
            result.push(bytes1[i]);
        }
    }

    str::from_utf8(&result).unwrap().to_string()
}

pub struct CommonInventoryChars;

impl Puzzle for CommonInventoryChars {
    fn solve(&self, input: &str) -> String {
        for line in input.lines() {
            for other_line in input.lines() {
                let chars = common_chars(line, other_line);

                if chars.len() == line.len() - 1 {
                    return chars;
                }
            }
        }

        unreachable!()
    }
}
