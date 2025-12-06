use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{enumerate, Itertools};
use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        //extract lines and seperate into the number lines and the line with the operators
        let lines = reader.lines();
        let mut numberLines: Vec<String> = lines.map(|l| l.unwrap()).collect();
        let operators = numberLines.pop().unwrap();
        //take operator line and convert into a vector
        let operators = operators.split_ascii_whitespace().collect_vec();
        //take first line of numbers and make them the initial values for each column
        let mut accumulators: Vec<i64> = numberLines[0]
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        //go through each of the other lines and extract the number before applying to the accumulator as applicable
        for line in numberLines.iter().skip(1) {
            let values: Vec<i64> = line
                .split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect_vec();
            for i in 0..values.len() {
                match operators[i] {
                    "+" => accumulators[i] += values[i],
                    "*" => accumulators[i] *= values[i],
                    _ => panic!(),
                }
            }
        }

        Ok(accumulators.iter().sum::<i64>() as usize)
    }

    assert_eq!(4277556, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        //extract lines and seperate out the operator line
        let mut numberLines: Vec<String> = lines.map(|l| l.unwrap()).collect();
        let operators = numberLines.pop().unwrap();
        //get the indexes of the operators for when split the number string later
        let operatorIndexes = operators
            .char_indices()
            .filter(|&(_, c)| matches!(c, '+' | '*'))
            .map(|(i, _)| i)
            .collect_vec();
        let operators = operators.split_ascii_whitespace().collect_vec();

        //extract each number from the number lines based on the indexes of the operators
        let mut numberLinesSplit: Vec<Vec<&str>> = Vec::new();
        for window in operatorIndexes.windows(2) {
            numberLinesSplit.push(
                numberLines
                    .iter()
                    .map(|s| &s[window[0]..window[1] - 1])
                    .collect(),
            );
        }
        //handle the last column as the foreach does not do so
        numberLinesSplit.push(
            numberLines
                .iter()
                .map(|s| &s[*operatorIndexes.last().unwrap()..])
                .collect(),
        );

        //go through each column and extract the number in position 0 of the column to make the initial value of the accumulator
        let mut accumulators: Vec<i64> = numberLinesSplit
            .iter()
            .map(|vec| {
                vec.iter()
                    .map(|s| s.chars().nth(0).unwrap())
                    .collect::<String>()
            })
            .collect_vec()
            .iter()
            .map(|s| s.trim().parse::<i64>().unwrap())
            .collect();

        //go through each column of the data
        for (index, item) in enumerate(numberLinesSplit) {
            //extract the part that is not used for the accumulator
            let substring = item
                .iter()
                .map(|s| s.chars().get(1..).collect::<String>())
                .collect_vec();
            let mut new_values: Vec<i64> = Vec::new();

            //find max_length in order to allow for padding of the strings and to go through each available number
            let max_length = substring.iter().max_by_key(|s| s.len()).unwrap().len();

            //extract the individual numbers and add them to a vector
            for i in 0..max_length {
                new_values.push(
                    substring
                        .iter()
                        .map(|s| {
                            format!("{:<width$}", s, width = max_length)
                                .chars()
                                .nth(i)
                                .unwrap()
                        })
                        .collect::<String>()
                        .trim()
                        .parse::<i64>()
                        .unwrap(),
                );
            }
            //go through each of the numbers found apply to accumulator based on the operator
            for value in new_values {
                match operators[index] {
                    "+" => accumulators[index] += value,
                    "*" => accumulators[index] *= value,
                    _ => panic!(),
                }
            }
        }

        Ok(accumulators.iter().sum::<i64>() as usize)
    }

    assert_eq!(3263827, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
