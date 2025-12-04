use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
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
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut counter = 0;
        //convert the input into a 2d array of characters
        for line in reader.lines() {
            let line = line?;
            map.push(line.chars().collect());
        }

        //go through each possible position in the map
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                //check if roll there and if not skip
                if map[i][j] != '@' {
                    continue;
                }
                //determine the slice to check based on position
                let mut istart = 0;
                let mut iend = map.len() - 1;
                let mut jstart = 0;
                let mut jend = map[i].len() - 1;
                if (i != 0) {
                    istart = i - 1;
                }
                if (j != 0) {
                    jstart = j - 1;
                }
                if (i != map.len() - 1) {
                    iend = i + 1
                }
                if (j != map[i].len() - 1) {
                    jend = j + 1
                }
                //extract the subset to check
                let subset: Vec<Vec<char>> = map
                    .iter()
                    .get(istart..=iend) //get applicable rows
                    .map(|row: &Vec<char>| {
                        row.iter()
                            .get(jstart..=jend) //get applicable columns
                            .cloned()
                            .collect::<Vec<char>>()
                    })
                    .collect();
                //convert 2d array into 1d and check how common the roll is
                let flattened_area: Vec<char> = subset.into_iter().flatten().collect();
                let occurance = flattened_area.iter().filter(|&&c| c == '@').count();
                if occurance < 5 {
                    counter += 1;
                }
            }
        }

        let answer = counter;
        Ok(answer)
    }

    assert_eq!(13, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut map: Vec<Vec<char>> = Vec::new();
        let mut counter = 0;
        //convert the input into a 2d array of characters
        for line in reader.lines() {
            let line = line?;
            map.push(line.chars().collect());
        }

        //loop till there is no change after an iteration
        loop {
            let counterStart = counter;
            //go through each possible position in the map
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    //check if roll there and if not skip
                    if map[i][j] != '@' {
                        continue;
                    }
                    //determine the slice to check based on position
                    let mut istart = 0;
                    let mut iend = map.len() - 1;
                    let mut jstart = 0;
                    let mut jend = map[i].len() - 1;
                    if (i != 0) {
                        istart = i - 1;
                    }
                    if (j != 0) {
                        jstart = j - 1;
                    }
                    if (i != map.len() - 1) {
                        iend = i + 1
                    }
                    if (j != map[i].len() - 1) {
                        jend = j + 1
                    }
                    //extract the subset to check
                    let subset: Vec<Vec<char>> = map
                        .iter()
                        .get(istart..=iend) //get  applicable rows
                        .map(|row: &Vec<char>| {
                            row.iter()
                                .get(jstart..=jend) //get applicable columns
                                .cloned()
                                .collect::<Vec<char>>()
                        })
                        .collect();
                    //convert 2d array into 1d and check how common the roll is
                    let flattened_area: Vec<char> = subset.into_iter().flatten().collect();
                    let occurance = flattened_area.iter().filter(|&&c| c == '@').count();
                    if occurance < 5 {
                        map[i][j] = '.'; //remove roll from map so can remove more afterwards
                        counter += 1;
                    }
                }
            }
            //if no new options have been found return the total detected
            if counter == counterStart {
                return Ok(counter);
            }
        }
    }

    assert_eq!(43, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
