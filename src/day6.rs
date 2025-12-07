use std::collections::BTreeMap;

use anyhow::Context;

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

#[derive(Debug)]
struct Problem {
    numbers: Vec<u16>,
    operation: Operation,
}

fn parse_problems(input: &str) -> anyhow::Result<Vec<Problem>> {
    let mut problem_builders: BTreeMap<usize, Vec<u16>> = BTreeMap::new();
    let mut problems = Vec::new();
    for line in input.lines() {
        if line.starts_with("*") || line.starts_with("+") {
            problems = parse_operation_line(&problem_builders, line)
                .context("failed to parse operation line")?;
        } else {
            for (i, number_str) in line.split_whitespace().into_iter().enumerate() {
                let number: u16 = number_str.parse().context("failed to parse number")?;
                problem_builders
                    .entry(i)
                    .and_modify(|x: &mut Vec<u16>| x.push(number))
                    .or_insert_with(|| vec![number]);
            }
        }
    }

    Ok(problems)
}

fn get_problems_grand_total(problems: &[Problem]) -> u64 {
    problems
        .iter()
        .map(|x| match x.operation {
            Operation::Add => x.numbers.iter().copied().map(|x| x as u64).sum::<u64>(),
            Operation::Mul => x.numbers.iter().copied().map(|x| x as u64).product::<u64>(),
        })
        .sum()
}

pub fn part1(input: &str) -> anyhow::Result<u64> {
    let problems = parse_problems(input).context("failed to parse problems")?;
    Ok(get_problems_grand_total(&problems))
}

fn find_max_digits_for_column(input: &str) -> Vec<usize> {
    let mut max_digits_per_columns: BTreeMap<usize, usize> = BTreeMap::new();
    for line in input.lines() {
        for (column, num) in line.split_whitespace().enumerate() {
            max_digits_per_columns
                .entry(column)
                .and_modify(|len| {
                    if num.len() > *len {
                        *len = num.len()
                    }
                })
                .or_insert(num.len());
        }
    }

    max_digits_per_columns.into_values().collect()
}

fn parse_problems_part2(input: &str) -> anyhow::Result<Vec<Problem>> {
    let mut problem_builders: BTreeMap<usize, Vec<u16>> = BTreeMap::new();
    let mut problems = Vec::new();

    let max_digits_per_column = find_max_digits_for_column(input);

    for line in input.lines() {
        if line.starts_with("*") || line.starts_with("+") {
            problems = parse_operation_line(&problem_builders, line)
                .context("failed to parse operation line")?;
        } else {
            let mut consumed_so_far = 0;
            for (column, chars_to_take) in max_digits_per_column.iter().enumerate() {
                let number = if consumed_so_far + chars_to_take > line.len() {
                    // the last number might not have enough digits, and we'll need to pad it
                    let num = &line[consumed_so_far..].trim_start();
                    let num_missing = chars_to_take - num.len();
                    let num: u16 = num
                        .parse()
                        .with_context(|| format!("failed to parse number: {num}"))?;
                    num * 10u16.pow(num_missing as u32)
                } else {
                    let num = &line[consumed_so_far..consumed_so_far + chars_to_take];
                    let count_zeros_to_add =
                        num.chars().rev().take_while(|c| c.is_whitespace()).count();
                    let num = num.trim();
                    // +1 for the whitespace
                    consumed_so_far += chars_to_take + 1;
                    let num: u16 = num
                        .parse()
                        .with_context(|| format!("failed to parse number: {num}"))?;
                    num * 10u16.pow(count_zeros_to_add as u32)
                };

                problem_builders
                    .entry(column)
                    .and_modify(|x: &mut Vec<u16>| x.push(number))
                    .or_insert_with(|| vec![number]);
            }
        }
    }

    Ok(problems)
}

fn parse_operation_line(
    problem_builders: &BTreeMap<usize, Vec<u16>>,
    line: &str,
) -> anyhow::Result<Vec<Problem>> {
    let mut problems = Vec::new();
    for (op, numbers) in line
        .split_whitespace()
        .into_iter()
        .zip(problem_builders.values())
    {
        let operation = match op {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            other => anyhow::bail!("{other} is not an op"),
        };

        problems.push(Problem {
            numbers: numbers.clone(),
            operation,
        });
    }

    Ok(problems)
}

pub fn part2(input: &str) -> anyhow::Result<u64> {
    let problems = parse_problems_part2(input).context("failed to parse problems")?;
    // now we have all the numbers aligned. We should get the grand total by going through the column

    let mut total_sum = 0;
    for problem in problems {
        let Some(max_digits) = problem.numbers.iter().map(|x| x.ilog10() + 1).max() else {
            anyhow::bail!("numbers are empty");
        };

        let mut total = match &problem.operation {
            Operation::Add => 0,
            Operation::Mul => 1,
        };

        for pow in 0..max_digits {
            let mut final_number = 0;
            let mut times = 0;
            for num in problem.numbers.iter().rev() {
                let pow = pow as u32;
                let to_add = ((num / 10u16.pow(pow)) % 10) * 10u16.pow(times);
                if to_add > 0 {
                    times += 1;
                }
                final_number += to_add;
            }
            match &problem.operation {
                Operation::Add => total += final_number as u64,
                Operation::Mul => total *= final_number as u64,
            }
        }
        total_sum += total;
    }

    Ok(total_sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {"
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +
        "};
        let result = part1(input).unwrap();
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {"
            123 328  51 64
             45 64  387 23
              6 98  215 314
            *   +   *   +
        "};
        let result = part2(input).unwrap();
        assert_eq!(result, 3263827);
    }
}
