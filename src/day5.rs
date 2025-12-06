use std::{ops::RangeInclusive, str::FromStr};

use anyhow::Context;

pub struct IngredientIdRange(RangeInclusive<u64>);

impl FromStr for IngredientIdRange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('-').fuse();
        let start = split
            .next()
            .context("no start")?
            .parse()
            .context("failed to parse start")?;
        let end = split
            .next()
            .context("no end")?
            .parse()
            .context("failed to parse end")?;
        Ok(Self(start..=end))
    }
}

pub struct IngredientDatabase {
    ingredient_id_ranges: Vec<IngredientIdRange>,
    ingredients: Vec<u64>,
}

impl IngredientDatabase {
    fn is_fresh(&self, id: &u64) -> bool {
        for range in &self.ingredient_id_ranges {
            if range.0.contains(id) {
                return true;
            }
        }

        false
    }
}

impl FromStr for IngredientDatabase {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ingredient_id_ranges: Vec<IngredientIdRange> = vec![];
        let mut ingredients: Vec<u64> = vec![];
        let mut found_blank = false;
        for line in s.lines() {
            if line.is_empty() {
                found_blank = true;
            } else if !found_blank {
                ingredient_id_ranges.push(line.parse().context("failed to parse range")?);
            } else {
                ingredients.push(line.parse().context("failed to parse id")?);
            }
        }

        Ok(Self {
            ingredient_id_ranges,
            ingredients,
        })
    }
}

pub fn part1(input: &str) -> anyhow::Result<u32> {
    let database: IngredientDatabase = input.parse().context("failed to parse database")?;

    let mut count = 0;
    for ingredient in &database.ingredients {
        if database.is_fresh(ingredient) {
            count += 1;
        }
    }

    Ok(count)
}

#[derive(Debug, Clone, Copy)]
struct MyRangeInclusive {
    start: u64,
    end: u64,
}

impl MyRangeInclusive {
    fn count(&self) -> u64 {
        self.end - self.start + 1
    }
}

fn do_ranges_intersect(r1: &MyRangeInclusive, r2: &MyRangeInclusive) -> bool {
    !(r1.end < r2.start || r2.end < r1.start)
}

fn combine_intersecting_ranges_single_iteration(
    ranges: Vec<MyRangeInclusive>,
) -> Vec<MyRangeInclusive> {
    let Some(mut current) = ranges.get(0).cloned() else {
        return vec![];
    };
    let mut new_ranges = Vec::new();

    for range in ranges.iter().skip(1) {
        if do_ranges_intersect(&current, range) {
            let new_min = std::cmp::min(current.start, range.start);
            let new_max = std::cmp::max(current.end, range.end);
            current = MyRangeInclusive {
                start: new_min,
                end: new_max,
            };
        } else {
            new_ranges.push(current);
            current = range.clone();
        }
    }
    new_ranges.push(current);

    new_ranges
}

fn combine_intersecting_ranges(mut ranges: Vec<MyRangeInclusive>) -> Vec<MyRangeInclusive> {
    ranges.sort_by_key(|x| (x.start, x.end));

    let mut current_len = ranges.len();
    loop {
        ranges = combine_intersecting_ranges_single_iteration(ranges);
        eprintln!("{:?}", ranges);
        if ranges.len() == current_len {
            return ranges;
        }
        current_len = ranges.len();
    }
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let database: IngredientDatabase = input.parse().context("failed to parse database")?;
    let ranges: Vec<_> = database
        .ingredient_id_ranges
        .into_iter()
        .map(|x| MyRangeInclusive {
            start: *x.0.start(),
            end: *x.0.end(),
        })
        .collect();

    let combined = combine_intersecting_ranges(ranges);
    Ok(combined.into_iter().map(|x| x.count()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        let result = part1(input).unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            3-5
            10-14
            16-20
            12-18

            1
            5
            8
            11
            17
            32
        "};
        let result = part2(input).unwrap();
        assert_eq!(result, 14);
    }
}
