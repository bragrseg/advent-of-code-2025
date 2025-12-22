use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST1: &str = "\
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
const TEST2: &str = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Debug, Clone)]
    struct Paths {
        input: String,
        output: Vec<String>,
    }

    //recursive function that goes down to out then back up to get the different paths
    fn possiblePaths1(options: &Vec<Paths>, input: String) -> usize {
        let mut counter = 0;
        let outputs = options
            .iter()
            .filter(|p| p.input == input)
            .collect::<Vec<_>>();
        for output in outputs[0].output.iter() {
            if output == "out" {
                counter += 1;
            } else {
                counter += possiblePaths1(&options, output.clone());
            }
        }
        counter
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut initialPoints: Vec<String> = Vec::new();
        let mut paths: Vec<Paths> = Vec::new();
        //read in file and extract the inputs and outputs for each line
        for line in reader.lines() {
            let line = line?;
            let in_out = line
                .split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let input = in_out[0].to_string();
            let output = in_out[1]
                .split(" ")
                .skip(1)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            if input == "you" {
                initialPoints = output;
            } else {
                paths.push(Paths { input, output });
            }
        }
        //run through each initial point in the path and determine the number of options each one has
        let count = initialPoints
            .iter()
            .map(|s| possiblePaths1(&paths, s.to_string()))
            .collect::<Vec<usize>>()
            .iter()
            .sum::<usize>();
        Ok(count)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(5, part1(BufReader::new(TEST1.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut initialPoints: Vec<String> = Vec::new();
        let mut paths: Vec<Paths> = Vec::new();
        let mut count = 0;

        //read in file and extract the inputs and outputs for each line
        for line in reader.lines() {
            let line = line?;
            let in_out = line
                .split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let input = in_out[0].to_string();
            let output = in_out[1]
                .split(" ")
                .skip(1)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            if input == "svr" {
                initialPoints = output;
            } else {
                paths.push(Paths { input, output });
            }
        }
        //valid,only dac, only fft, none
        let mut values: Vec<(usize, usize, usize, usize)> = vec![(0, 0, 0, 0); paths.len()];

        //determine the intiial indexes for the paths and add values to values vector to allow for path tracing
        let initialIndexes = paths
            .iter()
            .enumerate()
            .filter(|(_, p)| initialPoints.contains(&p.input))
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        initialIndexes.iter().for_each(|&i| values[i].3 = 1);

        //one iteration per item in paths (prevent infinite loops without having to check the values in the vectors)
        for i in 0..paths.len() {
            //create new vector to hold values after current iteration
            let mut newValues: Vec<(usize, usize, usize, usize)> = vec![(0, 0, 0, 0); paths.len()];
            for j in 0..paths.len() {
                //extract info for the path being currently checked
                let path: &Paths = paths.get(j).unwrap();
                let input = path.input.clone();
                let output = path.output.clone();
                //go through each possible out for the path
                for choice in output {
                    //if it is the out value then extract it from the vector and add to the counter
                    if choice == "out" {
                        count = count + values[j].0;
                        continue;
                    }
                    //take the path counts and place in the next value in the vector, move placement of values as applicable
                    let newIndex = paths.iter().position(|p| p.input == choice).unwrap();
                    if input == "dac" {
                        newValues[newIndex].0 += values[j].0 + values[j].2;
                        newValues[newIndex].1 += values[j].1 + values[j].3;
                    } else if input == "fft" {
                        newValues[newIndex].0 += values[j].0 + values[j].1;
                        newValues[newIndex].2 += values[j].2 + values[j].3;
                    } else {
                        newValues[newIndex].0 += values[j].0;
                        newValues[newIndex].1 += values[j].1;
                        newValues[newIndex].2 += values[j].2;
                        newValues[newIndex].3 += values[j].3;
                    }
                }
            }
            values = newValues;
        }
        Ok(count)
    }

    assert_eq!(2, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
