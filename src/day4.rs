use std::str::FromStr;

use anyhow::Context;

enum Cell {
    Paper,
    Nothing,
}

struct Grid {
    rows: Vec<Vec<Cell>>,
}

impl Grid {
    fn get_accessible_papers(&self) -> Vec<(usize, usize)> {
        let mut accessible = vec![];
        for (row_index, row) in self.rows.iter().enumerate() {
            for (column_index, cell) in row.iter().enumerate() {
                let Cell::Paper = cell else {
                    continue;
                };

                let positions = [
                    (1, 1),
                    (1, 0),
                    (0, 1),
                    (-1, -1),
                    (-1, 0),
                    (0, -1),
                    (1, -1),
                    (-1, 1),
                ];
                let mut num_adjecent_papers = 0;
                for position in positions {
                    let row_to_check = row_index as i32 + position.0;
                    let column_to_check = column_index as i32 + position.1;
                    if row_to_check < 0 || column_to_check < 0 {
                        continue;
                    }

                    if let Some(adjecent_row) = self.rows.get(row_to_check as usize)
                        && let Some(value) = adjecent_row.get(column_to_check as usize)
                        && let Cell::Paper = value
                    {
                        num_adjecent_papers += 1;
                    }
                    if num_adjecent_papers > 3 {
                        break;
                    }
                }

                if num_adjecent_papers < 4 {
                    accessible.push((row_index, column_index));
                }
            }
        }

        accessible
    }

    fn remove_papers(&mut self, papers: &[(usize, usize)]) {
        for (row_index, column_index) in papers {
            let Some(row) = self.rows.get_mut(*row_index) else {
                continue;
            };
            let Some(value) = row.get_mut(*column_index) else {
                continue;
            };

            *value = Cell::Nothing;
        }
    }
}

impl FromStr for Grid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        for line in s.lines() {
            let mut row = Vec::with_capacity(line.len());
            for s in line.chars() {
                let cell = match s {
                    '@' => Cell::Paper,
                    '.' => Cell::Nothing,
                    other => anyhow::bail!("{other} is not a valid cell"),
                };
                row.push(cell);
            }
            rows.push(row);
        }

        Ok(Self { rows })
    }
}

pub fn part1(input: &str) -> anyhow::Result<usize> {
    let grid: Grid = input.parse().context("failed to parse grid")?;
    Ok(grid.get_accessible_papers().len())
}

pub fn part2(input: &str) -> anyhow::Result<usize> {
    let mut grid: Grid = input.parse().context("failed to parse grid")?;
    let mut num_accessible = 0;
    let mut accessible = grid.get_accessible_papers();
    while accessible.len() > 0 {
        num_accessible += accessible.len();
        grid.remove_papers(&accessible);
        accessible = grid.get_accessible_papers();
    }
    Ok(num_accessible)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};
        let result = part1(input).unwrap();
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        "};
        let result = part2(input).unwrap();
        assert_eq!(result, 43);
    }
}
