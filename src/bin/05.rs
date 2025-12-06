use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
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
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut counter = 0;
        let mut ranges: Vec<(i64, i64)> = Vec::new();
        let lines = reader.lines().collect_vec();
        //find empty line in the input
        let emptyLine = lines
            .iter()
            .position(|line| line.as_ref().unwrap().trim().is_empty())
            .unwrap();
        //get vector of ranges from before the empty line
        let rangeLines = lines.get(..emptyLine).unwrap();
        for range in rangeLines {
            let parts = range.as_ref().unwrap().split('-').collect_vec();
            ranges.push((
                parts[0].parse::<i64>().unwrap(),
                parts[1].parse::<i64>().unwrap(),
            ));
        }
        //get vector or numbers after empty line and determine if valid
        let itemLines = lines.get(emptyLine + 1..).unwrap();
        for item in itemLines {
            let itemValue = item.as_ref().unwrap().parse::<i64>().unwrap();
            let mut valid: bool = false;
            for range in ranges.iter() {
                if itemValue >= range.0 && itemValue <= range.1 {
                    valid = true;
                    break;
                }
            }
            if valid {
                counter += 1;
            }
        }
        Ok(counter)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut counter = 0;
        let mut ranges: Vec<(i64, i64)> = Vec::new();
        let lines = reader.lines().collect_vec();
        //determine empty line in the input
        let emptyLine = lines
            .iter()
            .position(|line| line.as_ref().unwrap().trim().is_empty())
            .unwrap();
        //get the lines before the empty line
        let rangeLines = lines.get(..emptyLine).unwrap();
        for range in rangeLines {
            //extract the start and end and determine how it interacts with the other ranges found
            let parts = range.as_ref().unwrap().split('-').collect_vec();
            let mut start = parts[0].parse::<i64>()?;
            let mut end = parts[1].parse::<i64>()?;
            let mut toRemove: Vec<usize> = Vec::new();
            for (idx, tuple) in ranges.clone().iter().enumerate() {
                //new if both before and after current one
                if start <= tuple.0 && tuple.1 <= end {
                    toRemove.push(idx);
                }
                //new if before start but not after end
                else if start <= tuple.0 && end <= tuple.1 && end >= tuple.0 {
                    end = tuple.1;
                    toRemove.push(idx);
                }
                //new if start before end but end after end
                else if start >= tuple.0 && end >= tuple.1 && start <= tuple.1 {
                    start = tuple.0;
                    toRemove.push(idx);
                }
                //new is completely within current
                else if start >= tuple.0 && end <= tuple.1 {
                    start = tuple.0;
                    end = tuple.1;
                    toRemove.push(idx);
                }
            }
            toRemove.reverse();
            for idx in toRemove {
                ranges.remove(idx);
            }
            ranges.push((start, end));
        }
        //do counter on all the ranges found
        ranges.sort_by(|a, b| a.0.cmp(&b.0));
        for item in ranges {
            counter += item.1 - item.0 + 1;
        }
        println!("{:?}", counter);
        Ok(counter as usize)
    }

    assert_eq!(14, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
