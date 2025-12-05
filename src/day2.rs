use std::{ops::RangeInclusive, str::FromStr};

use anyhow::Context;

/// An invalid ID is a number which is made only of some sequence of digits repeated twice
fn is_valid_id(num: u64) -> bool {
    let num_digits = num.ilog10() + 1;
    if num_digits % 2 != 0 {
        // odd number of digits, so we can't have a repeating sequence twice
        return true;
    }

    let pow = 10u64.pow(num_digits / 2);
    let higher_digits = num / pow;
    let lower_digits = num % pow;
    higher_digits != lower_digits
}

/// Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice
fn is_valid_id_part2(num: u64) -> bool {
    // eprintln!("num: {num}");
    let num_digits = num.ilog10() + 1;

    if num < 10 {
        // a single digit isn't repeated at least twice
        return true;
    }

    for digits_to_check in 1..num_digits {
        if num_digits % digits_to_check != 0 {
            // if we can't split to repeated sequences it can't be an invalid ID
            continue;
        }
        // eprintln!("digits_to_check: {digits_to_check}");

        let modulo = 10u64.pow(digits_to_check);
        let sequence = num % modulo;
        // eprintln!("sequence: {sequence}");
        let mut found_different = false;
        let mut i = 1;
        loop {
            let pow = 10u64.pow(digits_to_check * i);
            if pow > num {
                break;
            }
            // eprintln!("pow: {pow}, checking: {}", (num / pow) % modulo);
            if (num / pow) % modulo != sequence {
                found_different = true;
                break;
            }
            i += 1;
        }

        if !found_different {
            return false;
        }
    }

    true
}

struct IdRange(RangeInclusive<u64>);

impl IdRange {
    pub fn new(range: RangeInclusive<u64>) -> Self {
        Self(range)
    }

    pub fn sum_invalid_ids(self) -> u64 {
        let mut sum_invalid = 0;
        for num in self.0 {
            if !is_valid_id(num) {
                sum_invalid += Into::<u64>::into(num);
            }
        }

        sum_invalid
    }

    pub fn sum_invalid_ids_part2(self) -> u64 {
        let mut sum_invalid = 0;
        for num in self.0 {
            if !is_valid_id_part2(num) {
                sum_invalid += Into::<u64>::into(num);
            }
        }

        sum_invalid
    }
}

impl FromStr for IdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // <num>-<num>
        let mut split = s.split('-').fuse();
        let start = split
            .next()
            .context("no start number")?
            .parse()
            .with_context(|| format!("failed to parse start from: {s}"))?;
        let end = split
            .next()
            .context("no end number")?
            .parse()
            .with_context(|| format!("failed to parse end: {s}"))?;
        Ok(Self::new(start..=end))
    }
}

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let mut sum_invalid = 0;
    for possible_range in input.split(',') {
        let range: IdRange = possible_range.parse().context("failed to parse range")?;
        sum_invalid += range.sum_invalid_ids();
    }

    Ok(sum_invalid)
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let mut sum_invalid = 0;
    for possible_range in input.split(',') {
        let range: IdRange = possible_range.parse().context("failed to parse range")?;
        sum_invalid += range.sum_invalid_ids_part2();
    }

    Ok(sum_invalid)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124"
        };
        let result = part1(input).unwrap();
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_is_valid_id_part2() {
        assert!(!is_valid_id_part2(2121212121));
        assert!(is_valid_id_part2(2121212120));
        assert!(is_valid_id_part2(3));
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124"
        };
        let result = part2(input).unwrap();
        assert_eq!(result, 4174379265);
    }
}
