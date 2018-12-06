use std::cmp;

use regex::Regex;

use Puzzle;

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Claim {
    fn from_line(line: &str) -> Option<Claim> {
        lazy_static! {
            static ref CLAIM_REGEX: Regex =
                Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let captures = CLAIM_REGEX.captures(line).unwrap();
        Some(Claim {
            id: captures.get(1)?.as_str().parse().unwrap(),
            x: captures.get(2)?.as_str().parse().unwrap(),
            y: captures.get(3)?.as_str().parse().unwrap(),
            width: captures.get(4)?.as_str().parse().unwrap(),
            height: captures.get(5)?.as_str().parse().unwrap(),
        })
    }
}

pub struct OverlappingFabricClaims;

impl Puzzle for OverlappingFabricClaims {
    fn solve(&self, input: &str) -> String {
        let mut claims: Vec<Claim> = vec![];
        let mut fabric_width = 0;
        let mut fabric_height = 0;
        for line in input.lines() {
            let claim = Claim::from_line(line).unwrap();
            fabric_width = cmp::max(fabric_width, claim.x + claim.width);
            fabric_height = cmp::max(fabric_height, claim.y + claim.height);
            claims.push(claim);
        }

        let mut counts: Vec<usize> = vec![0; fabric_width * fabric_height];

        for claim in &claims {
            for sub_x in claim.x..claim.x + claim.width {
                for sub_y in claim.y..claim.y + claim.height {
                    let index = sub_x * fabric_height + sub_y;
                    counts[index] += 1;
                }
            }
        }

        let nonoverlapping_claim = {
            let mut maybe_claim = None;

            'claim: for claim in &claims {
                for sub_x in claim.x..claim.x + claim.width {
                    for sub_y in claim.y..claim.y + claim.height {
                        let index = sub_x * fabric_height + sub_y;
                        if counts[index] > 1 {
                            continue 'claim;
                        }
                    }
                }

                maybe_claim = Some(claim)
            }

            maybe_claim.unwrap()
        };

        let overlap_count = counts.iter().filter(|x| **x > 1).count();
        format!(
            "overlapping: {}\nnon-overlapping claim id: {}",
            overlap_count, nonoverlapping_claim.id
        )
    }
}
