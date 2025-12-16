use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut positions: Vec<(i64, i64)> = Vec::new();
        let mut answer = 0;
        for line in reader.lines() {
            let line = line?;
            let numbers = line
                .trim()
                .split(',')
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            positions.push((numbers[0], numbers[1]));
        }
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let area = ((positions[i].0 - positions[j].0).abs() + 1)
                    * ((positions[i].1 - positions[j].1).abs() + 1);
                if area > answer {
                    answer = area
                }
            }
        }
        Ok(answer as usize)
    }

    assert_eq!(50, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
