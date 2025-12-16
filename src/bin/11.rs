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
