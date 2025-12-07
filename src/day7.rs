use std::{
    collections::{HashMap, HashSet},
    mem,
    str::FromStr,
};

use anyhow::Context;

#[derive(Debug, Clone, Copy)]
enum Location {
    Empty,
    Splitter,
}

#[derive(Debug)]
struct Manifold {
    grid: Vec<Vec<Location>>,
    start: (usize, usize),
}

impl Manifold {
    fn get(&self, row_column: (usize, usize)) -> Option<Location> {
        self.grid
            .get(row_column.0)
            .and_then(|row| row.get(row_column.1))
            .copied()
    }
}

impl FromStr for Manifold {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut grid = Vec::new();
        for (row, line) in s.lines().enumerate() {
            let mut grid_row = Vec::with_capacity(line.len());
            for (column, c) in line.char_indices() {
                let location = match c {
                    '.' => Location::Empty,
                    '^' => Location::Splitter,
                    'S' => {
                        start = Some((row, column));
                        Location::Empty
                    }
                    other => anyhow::bail!("{other} is not a valid location"),
                };
                grid_row.push(location);
            }
            grid.push(grid_row);
        }

        Ok(Self {
            grid,
            start: start.context("no start found")?,
        })
    }
}

#[derive(Debug)]
struct ManifoldWalker<'a> {
    manifold: &'a Manifold,
    beams: Vec<(usize, usize)>,
}

impl<'a> ManifoldWalker<'a> {
    fn new(manifold: &'a Manifold) -> Self {
        Self {
            manifold,
            beams: vec![manifold.start],
        }
    }

    fn step(&mut self) -> Option<usize> {
        let mut splits = 0;
        let mut new_beams = HashSet::new();
        let current_beams = mem::take(&mut self.beams);
        for beam in current_beams {
            let (next_row, next_column) = (beam.0 + 1, beam.1);

            // we assume the final row can't have a splitter at the edge
            if next_row == self.manifold.grid.len() {
                return None;
            }
            let next = (next_row, next_column);

            let Some(location) = self.manifold.get(next) else {
                continue;
            };

            match location {
                Location::Empty => {
                    new_beams.insert(next);
                }
                Location::Splitter => {
                    splits += 1;
                    new_beams.insert((next_row, next_column + 1));
                    new_beams.insert((next_row, next_column - 1));
                }
            };
        }
        self.beams = new_beams.into_iter().collect();

        Some(splits)
    }
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let manifold: Manifold = input.parse().context("failed to parse input")?;
    let mut walker = ManifoldWalker::new(&manifold);
    let mut splits_sum = 0;
    while let Some(split) = walker.step() {
        splits_sum += split;
    }
    Ok(splits_sum)
}

#[derive(Debug)]
struct QuantumBeam {
    at: (usize, usize),
    num_timelines: usize,
}

#[derive(Debug)]
struct QuantumManifoldWalker<'a> {
    manifold: &'a Manifold,
    beams: Vec<QuantumBeam>,
}

impl<'a> QuantumManifoldWalker<'a> {
    fn new(manifold: &'a Manifold) -> Self {
        Self {
            manifold,
            beams: vec![QuantumBeam {
                at: manifold.start,
                num_timelines: 1,
            }],
        }
    }

    fn step(&mut self) -> Option<usize> {
        let mut splits = 0;
        let mut new_beams = Vec::new();
        let current_beams = mem::take(&mut self.beams);
        for beam in current_beams {
            let (next_row, next_column) = (beam.at.0 + 1, beam.at.1);

            // we assume the final row can't have a splitter at the edge
            if next_row == self.manifold.grid.len() {
                return None;
            }
            let next = (next_row, next_column);

            let Some(location) = self.manifold.get(next) else {
                continue;
            };

            match location {
                Location::Empty => {
                    new_beams.push(QuantumBeam {
                        at: next,
                        num_timelines: beam.num_timelines,
                    });
                }
                Location::Splitter => {
                    splits += beam.num_timelines;
                    new_beams.push(QuantumBeam {
                        at: (next_row, next_column + 1),
                        num_timelines: beam.num_timelines,
                    });
                    new_beams.push(QuantumBeam {
                        at: (next_row, next_column - 1),
                        num_timelines: beam.num_timelines,
                    });
                }
            };
        }
        let mut new_beams_folded: HashMap<(usize, usize), QuantumBeam> = HashMap::new();
        for new_beam in new_beams {
            new_beams_folded
                .entry(new_beam.at)
                .and_modify(|x| x.num_timelines += new_beam.num_timelines)
                .or_insert(new_beam);
        }

        self.beams = new_beams_folded.into_values().collect();

        Some(splits)
    }
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let manifold: Manifold = input.parse().context("failed to parse input")?;
    let mut walker = QuantumManifoldWalker::new(&manifold);
    let mut splits_sum = 1;
    while let Some(split) = walker.step() {
        splits_sum += split;
    }
    Ok(splits_sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............
    "};

    #[test]
    fn test_part1() {
        let result = part1(INPUT).unwrap();
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT).unwrap();
        assert_eq!(result, 40);
    }
}
