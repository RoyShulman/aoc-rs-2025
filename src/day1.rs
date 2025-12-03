use anyhow::Context;

/// A dial has a maximum value of 0..100
struct Dial(u8);

impl Dial {
    const MAX_VALUE: u8 = 100;
    const fn new(value: u8) -> Option<Self> {
        if value >= Self::MAX_VALUE {
            return None;
        }
        Some(Self(value))
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn add_assign_count_saturations(&mut self, rhs: i16) -> u16 {
        let (normalized_rhs, mut num_saturations) = if rhs < 0 {
            let full_rotations: u16 = rhs.unsigned_abs() / Into::<u16>::into(Self::MAX_VALUE);
            let complement_rhs =
                (rhs + (Self::MAX_VALUE as u16 * (full_rotations + 1)) as i16) as u16;
            (complement_rhs, full_rotations)
        } else {
            // we know it's above 0, and `i16` always fits into a `u16`
            (rhs as u16, rhs as u16 / Self::MAX_VALUE as u16)
        };

        let modulo_rhs = normalized_rhs % Self::MAX_VALUE as u16;
        let result = ((self.0 as u16 + modulo_rhs) % Self::MAX_VALUE as u16) as u8;
        if (self.0 != 0 && result != 0)
            && ((result > self.0 && rhs < 0) || (result < self.0 && rhs > 0))
        {
            num_saturations += 1;
        }

        self.0 = result;
        num_saturations
    }
}

impl std::ops::AddAssign<i16> for Dial {
    fn add_assign(&mut self, rhs: i16) {
        // ignore num rotations for part1
        self.add_assign_count_saturations(rhs);
    }
}

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let mut num_zero = 0;
    let mut dial = Dial::new(50).expect("50 is a valid dial starting value");
    for line in input.lines() {
        let mut chars = line.chars().fuse();
        let direction = chars.next().context("failed to find direction")?;
        let distance: i16 = chars
            .collect::<String>()
            .parse()
            .context("failed to parse distance")?;

        let distance = match direction {
            'L' => -distance,
            'R' => distance,
            other => anyhow::bail!("invalid direction: {other}"),
        };
        dial += distance;
        if dial.is_zero() {
            num_zero += 1;
        }
    }
    Ok(num_zero)
}

pub fn part2(input: &str) -> anyhow::Result<u32> {
    let mut num_rotations = 0;
    let mut dial = Dial::new(50).expect("50 is a valid dial starting value");
    for line in input.lines() {
        let mut chars = line.chars().fuse();
        let direction = chars.next().context("failed to find direction")?;
        let distance: i16 = chars
            .collect::<String>()
            .parse()
            .context("failed to parse distance")?;

        let distance = match direction {
            'L' => -distance,
            'R' => distance,
            other => anyhow::bail!("invalid direction: {other}"),
        };
        num_rotations += dial.add_assign_count_saturations(distance) as u32;
        if dial.is_zero() {
            num_rotations += 1;
        }
    }
    Ok(num_rotations)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::day1::{Dial, part1, part2};

    #[test]
    fn test_dial() {
        let mut dial = Dial::new(11).unwrap();
        dial += 8;
        assert_eq!(dial.0, 19);
        dial += -19;
        assert_eq!(dial.0, 0);

        let mut dial = Dial::new(0).unwrap();
        dial += -1;
        assert_eq!(dial.0, 99);
        dial += 1;
        assert_eq!(dial.0, 0);

        let mut dial = Dial::new(5).unwrap();
        dial += -10;
        assert_eq!(dial.0, 95);
        dial += 5;
        assert_eq!(dial.0, 0);
    }

    #[test]
    fn test_day1() {
        let input = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82"
        };
        let result = part1(input).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_day2() {
        let input = indoc! {"
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82"
        };
        let result = part2(input).unwrap();
        assert_eq!(result, 6);
    }
}
