use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{enumerate, Itertools};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
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
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut counter = 0;
        let mut lines = reader.lines();
        let initial_position = lines.next().unwrap()?.find("S").unwrap();
        let mut beam_positions: Vec<i32> = vec![initial_position as i32];

        // go through each line of the file and check it
        for line in lines {
            let line = line?;
            // find the positions of all the splitters in the line
            let splitters = line
                .char_indices()
                .filter_map(|(i, c)| if c == '^' { Some(i as i32) } else { None })
                .collect::<Vec<i32>>();
            let mut new_positions: Vec<i32> = Vec::new();
            // check each splitter for if a beam was hitting it and split it
            for position in splitters.clone() {
                if beam_positions.contains(&position) {
                    new_positions.push(position as i32 - 1);
                    new_positions.push(position as i32 + 1);
                    counter = counter + 1;
                }
            }

            // add the new beams and remove any that have hit a splitter
            beam_positions.append(&mut new_positions);
            beam_positions.dedup();
            beam_positions = beam_positions
                .into_iter()
                .filter(|x| !splitters.iter().contains(x))
                .collect();
        }

        Ok(counter)
    }

    assert_eq!(21, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut lines = reader.lines();
        let initial_position = lines.next().unwrap()?.find("S").unwrap();
        let lines = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
        // vector of the beam positions along with how many values are on each line
        let mut beam_positions: Vec<i64> = vec![initial_position as i64];
        let mut counts: Vec<i64> = vec![0; lines[0].len()];
        // add the initial count for the start
        counts[initial_position] += 1;
        for line in lines {
            // find splitters for the current line
            let splitters = line
                .char_indices()
                .filter_map(|(i, c)| if c == '^' { Some(i as i64) } else { None })
                .collect::<Vec<i64>>();
            let mut new_positions: Vec<i64> = Vec::new();
            // check if each beam has hit a splitter and find new positions if so, also add counts to new lines if a valid line
            for position in splitters.clone() {
                if beam_positions.contains(&position) {
                    new_positions.push(position - 1);
                    new_positions.push(position + 1);
                    if !splitters.contains(&(position - 1)) {
                        counts[(position - 1) as usize] += counts[position as usize];
                    }
                    if !splitters.contains(&(position + 1)) {
                        counts[(position + 1) as usize] += counts[position as usize];
                    }
                }
            }
            // 0 out the lines that hit a splitter
            for i in splitters.clone() {
                counts[i as usize] = 0;
            }
            // clean up the beam positions to only have valid ones
            beam_positions.append(&mut new_positions);
            beam_positions.dedup();
            beam_positions = beam_positions
                .into_iter()
                .filter(|x| !splitters.iter().contains(x))
                .collect();
        }
        // sum the counts once at the bottom
        Ok(counts.iter().sum::<i64>() as usize)
    }

    assert_eq!(40, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
