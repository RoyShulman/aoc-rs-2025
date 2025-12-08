use std::{cmp::Reverse, collections::HashSet, str::FromStr};

use anyhow::Context;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    x: u32,
    y: u32,
    z: u32,
}

impl Location {
    fn distance(&self, other: &Location) -> u64 {
        (self.x as i64 - other.x as i64).pow(2) as u64
            + (self.y as i64 - other.y as i64).pow(2) as u64
            + (self.z as i64 - other.z as i64).pow(2) as u64
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct JunctionBox {
    location: Location,
}

impl FromStr for JunctionBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(",").fuse();
        let x: u32 = it
            .next()
            .context("no first number")?
            .parse()
            .context("failed to parse first number")?;
        let y: u32 = it
            .next()
            .context("no second number")?
            .parse()
            .context("failed to parse second number")?;
        let z: u32 = it
            .next()
            .context("no third number")?
            .parse()
            .context("failed to parse third number")?;
        Ok(Self {
            location: Location { x, y, z },
        })
    }
}

fn get_sorted_distances(boxes: &[JunctionBox]) -> Vec<(u64, (&JunctionBox, &JunctionBox))> {
    let mut distances = Vec::with_capacity(boxes.len() * boxes.len());
    for (i, junction_box) in boxes.iter().enumerate() {
        for other in boxes.iter().skip(i) {
            if junction_box == other {
                continue;
            }

            let distance = junction_box.location.distance(&other.location);
            distances.push((distance, (junction_box, other)));
        }
    }

    distances.sort_by_key(|(d, _)| *d);

    distances
}

fn merge_boxes<'a>(
    circuits: &mut Vec<HashSet<&'a JunctionBox>>,
    j1: &'a JunctionBox,
    j2: &'a JunctionBox,
) -> anyhow::Result<()> {
    let first_circuit = circuits.iter().position(|c| c.contains(&j1));
    let second_circuit = circuits.iter().position(|c| c.contains(&j2));

    if let Some(first) = first_circuit
        && let Some(second) = second_circuit
    {
        if second == first {
            // do nothing
            return Ok(());
        }
        if second > first {
            let second = circuits.remove(second);
            let first = circuits.get_mut(first).context("failed to get first")?;
            first.extend(second);
        } else {
            let first = circuits.remove(first);
            let second = circuits.get_mut(second).context("failed to get second")?;
            second.extend(first);
        }
    } else if let Some(first) = first_circuit {
        let c = circuits
            .get_mut(first)
            .context("failed to get first circuit")?;
        c.insert(j2);
    } else if let Some(second) = second_circuit {
        let c = circuits
            .get_mut(second)
            .context("failed to get second circuit")?;
        c.insert(j1);
    } else {
        circuits.push(HashSet::from_iter([j1, j2]));
    }

    Ok(())
}

pub fn part1(input: &str, num_connections: usize) -> anyhow::Result<usize> {
    let mut boxes = Vec::new();
    for line in input.lines() {
        let junction_box: JunctionBox = line.parse().context("failed to parse line")?;
        boxes.push(junction_box);
    }
    let distances = get_sorted_distances(&boxes);

    let mut circuits: Vec<HashSet<&JunctionBox>> = Vec::new();
    // for b in &boxes {
    //     circuits.push(HashSet::from_iter([b]));
    // }

    for (_, (j1, j2)) in distances.into_iter().take(num_connections) {
        merge_boxes(&mut circuits, j1, j2).context("failed to merged")?;
    }

    circuits.sort_by_key(|x| Reverse(x.len()));
    Ok(circuits.iter().take(3).map(|x| x.len()).product())
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let mut boxes = Vec::new();
    for line in input.lines() {
        let junction_box: JunctionBox = line.parse().context("failed to parse line")?;
        boxes.push(junction_box);
    }
    let distances = get_sorted_distances(&boxes);
    let mut circuits: Vec<HashSet<&JunctionBox>> = Vec::new();
    for b in &boxes {
        circuits.push(HashSet::from_iter([b]));
    }
    let mut distance_it = distances.into_iter();

    let mut result = None;
    while circuits.len() != 1 {
        let (_, (j1, j2)) = distance_it.next().context("no more boxes to connect")?;
        merge_boxes(&mut circuits, j1, j2).context("failed to merged")?;
        result = Some(j1.location.x as u64 * j2.location.x as u64);
    }

    let result = result.context("no merges happened")?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        162,817,812
        57,618,57
        906,360,560
        592,479,940
        352,342,300
        466,668,158
        542,29,236
        431,825,988
        739,650,466
        52,470,668
        216,146,977
        819,987,18
        117,168,530
        805,96,715
        346,949,466
        970,615,88
        941,993,340
        862,61,35
        984,92,344
        425,690,689
    "};

    #[test]
    fn test_part1() {
        let result = part1(INPUT, 10).unwrap();
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, 25272);
    }
}
