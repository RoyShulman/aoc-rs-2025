use std::{cmp::Reverse, str::FromStr};

use anyhow::Context;

struct PowerBank {
    digits: Vec<u8>,
}

impl PowerBank {
    fn find_max_from_index(&self, from: usize, to: usize) -> anyhow::Result<(usize, u8)> {
        // we don't use `.max()` since it returns the last one if multiple are equal
        let mut max_num_and_index = None;
        for (i, d) in self.digits[from..to].iter().enumerate() {
            if let Some((_, max_d)) = &max_num_and_index {
                if d > max_d {
                    max_num_and_index = Some((i, *d));
                }
            } else {
                max_num_and_index = Some((i, *d));
            }
        }

        let Some(max_num_and_index) = max_num_and_index else {
            anyhow::bail!("digits are empty");
        };

        Ok(max_num_and_index)
    }

    fn sum_top_2(&self) -> anyhow::Result<u16> {
        // we always want to first find the highest number that appears first,
        // since no matter what it'll be higher than even if we find a 9
        // that is after it
        let (i, tens) = self
            .find_max_from_index(0, self.digits.len() - 1)
            .context("couldn't find top 1")?;
        let (_, ones) = self
            .find_max_from_index(i + 1, self.digits.len())
            .context("couldn't find second")?;
        Ok(tens as u16 * 10 + ones as u16)
    }

    fn sum_top_12(&self) -> anyhow::Result<u64> {
        let mut sum = 0;
        let mut from = 0;
        for i in 0..12 {
            let (next_from, value) = self
                .find_max_from_index(from, self.digits.len() - 11 + i)
                .with_context(|| format!("failed to find max for {i}"))?;
            from = from + next_from + 1;
            sum += value as u64 * 10u64.pow((11 - i) as u32);
        }

        Ok(sum)
    }
}

impl FromStr for PowerBank {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digits = Vec::with_capacity(s.len());
        for c in s.chars().into_iter() {
            let d = c.to_digit(10).context("failed to convert char to digit")?;
            if d >= u8::MAX as u32 {
                anyhow::bail!("digit {d} is above the max value");
            }
            let d = d as u8;
            digits.push(d);
        }

        Ok(Self { digits })
    }
}

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let power_bank: PowerBank = line.parse().context("failed to parse line")?;
        sum += power_bank.sum_top_2().context("failed to sum top 2")? as u32;
    }

    Ok(sum)
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let power_bank: PowerBank = line.parse().context("failed to parse line")?;
        sum += power_bank.sum_top_12().context("failed to sum top 12")? as u64;
    }

    Ok(sum)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111"};
        assert_eq!(part1(input).unwrap(), 357);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            987654321111111
            811111111111119
            234234234234278
            818181911112111"};
        assert_eq!(part2(input).unwrap(), 3121910778619);
    }
}
