use adv_code_2025::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
987654321111111
811111111111119
234234234234278
818181911112111
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        let mut answer: usize = 0;
        //process through each line of input
        for line in lines {
            let line = line?;
            let letters = line.chars().collect::<Vec<_>>();
            //determine the first position of the highest valid start number
            let startvalue = letters[0..letters.len() - 1].iter().max().unwrap();
            let startposition = letters.iter().position(|l| *l == *startvalue).unwrap();
            let endvalue = letters[startposition + 1..].iter().max().unwrap();
            //add highest valid number to overall value
            answer = answer
                + format!("{}{}", startvalue, endvalue)
                    .parse::<i32>()
                    .unwrap() as usize;
        }
        Ok(answer)
    }

    assert_eq!(357, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    //find the highest possible value fromm the given characters for the length provided
    fn get_highest(chars: Vec<char>, length: usize) -> String {
        //deal with instance where just want the highest available character
        if length == 1 {
            let highest: String = chars
                .iter()
                .map(|c| c.to_string())
                .unique() //remove duplicates
                .sorted() //sort and reverse to find highest
                .rev()
                .take(1) //take the first in the iter and assign to highest
                .collect();

            return highest;
        }
        //determine the value and position of the highest valid number for the start of the return string based on the length provided
        let startvalue = chars[0..(chars.len() - (length - 1))].iter().max().unwrap();
        let startposition = chars.iter().position(|l| *l == *startvalue).unwrap();
        //use recursion to get the part of the string after the highest valid
        let option = get_highest(chars[startposition + 1..].to_vec(), length - 1);
        let highest = format!("{}{}", chars[startposition], option); //merge the highest valid with the string provided for the rest
        highest
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader.lines();
        let mut answer: usize = 0;
        //process through each line of input
        for line in lines {
            let line = line?;
            let letters = line.chars().collect::<Vec<_>>();
            //determine the first position of the highest valid start number
            let startvalue = letters[0..letters.len() - 11].iter().max().unwrap();
            let startposition = letters.iter().position(|l| *l == *startvalue).unwrap();

            //determine the rest of the string for highest value
            let combo = get_highest(letters.get(startposition..).unwrap().to_vec(), 12);
            let number = combo.parse::<i64>()?;

            //add highest valid value to overall value
            answer = answer + number as usize;
        }
        Ok(answer)
    }

    assert_eq!(3121910778619, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
