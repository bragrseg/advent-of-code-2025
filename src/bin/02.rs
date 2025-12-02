use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2025::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124
"; 

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let lines = reader.lines();
        let mut input:String = "".to_string();
        for line in lines{
            input = input+ &line?;
        }
        let items = input.split(",").collect::<Vec<&str>>();
        for item in items{
            let range = item.split("-").collect::<Vec<&str>>();
            let start = range[0].parse::<usize>().unwrap();
            let end = range[1].parse::<usize>().unwrap();
            for i in start..=end{
                let string = i.to_string();
                let string = string.as_str();
                if string.len()%2==0{
                    if &string[0..string.len()/2]== &string[string.len()/2..string.len()] {
                        answer += string.parse::<usize>()?;
                    }
                }
            }
        }

        Ok(answer)
    }
    
    assert_eq!(1227775554, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut answer = 0;
        let lines = reader.lines();
        let mut input:String = "".to_string();
        for line in lines{
            input = input+ &line?;
        }
        let items = input.split(",").collect::<Vec<&str>>();
        for item in items{
            let range = item.split("-").collect::<Vec<&str>>();
            let start = range[0].parse::<usize>().unwrap();
            let end = range[1].parse::<usize>().unwrap();
            for i in start..=end{
                let string = i.to_string();
                let string = string.as_str();
                for j in 1..string.len()/2+1{
                    if string.len()%j==0{
                        let amount = &string[0..j];
                        let mut valid = true;
                        for i in 1..string.len()/j{
                            if amount != &string[i*j..(i+1)*j]{
                                valid = false;
                                break;
                            }
                        }
                        if valid{
                            answer += string.parse::<usize>()?;
                            break;
                        }
                    }
                }

                // if string.len()%2==0{
                //     if &string[0..string.len()/2]== &string[string.len()/2..string.len()] {
                //         answer += string.parse::<usize>()?;
                //     }
                // }
            }
        }

        Ok(answer)
    }

    assert_eq!(4174379265, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
