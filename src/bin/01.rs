use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2025::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

const INITIAL_POSITION:i32 = 50;
const ROTOR_SIZE:i32 = 100;
const ROTOR_CLICK_POSITION:i32 = 0;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let mut position:i32 = INITIAL_POSITION;
        let mut count = 0;
        //read through each line in the input
        for line in reader.lines() {
            let line = line?;
            //check if the rotation is to the left
            if(line.starts_with("L")){
                let amount:i32 = line.split("L").nth(1).unwrap().parse()?;
                position = (position - amount)%ROTOR_SIZE;
            }
            //check if the rotation is to the right
            else if line.starts_with("R") {
                let amount:i32 = line.split("R").nth(1).unwrap().parse()?;
                position = (position+amount)%ROTOR_SIZE;
            }
            //determine if the rotor is in the 0 position
            if(position==ROTOR_CLICK_POSITION)
            {
                count += 1;
            }
        }
        let answer = count;
        Ok(answer)
    }


    assert_eq!(3, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    //take an amount and extract the number of full rotations in it
    fn div_amount(amount: &mut i32) ->usize{
        let div = *amount / ROTOR_SIZE;
        *amount = *amount%ROTOR_SIZE;
        div as usize
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut position:i32 = INITIAL_POSITION;
        let mut count:usize = 0;
        //read through each line in the input
        for line in reader.lines() {
            let line = line?;
            //check if the rotation is to the left
            if(line.starts_with("L")){
                let mut amount:i32 = line.split("L").nth(1).unwrap().parse()?;
                if amount > ROTOR_SIZE {
                    count += div_amount(&mut amount);
                }
                if amount > position && position != 0{
                    count += 1;
                }
                position = (position - amount)%ROTOR_SIZE;
                while position<0{
                    position = position + ROTOR_SIZE;
                }
            }
            //check if the rotation is to the right
            else if line.starts_with("R") {
                let mut amount:i32 = line.split("R").nth(1).unwrap().parse()?;
                if amount>ROTOR_SIZE{
                    count += div_amount(&mut amount);
                }
                if amount + position > ROTOR_SIZE && position != 0{
                    count+=1;
                }
                position = (position+amount)%ROTOR_SIZE;
            }
            //determine if the rotor is in the 0 position
            if(position==ROTOR_CLICK_POSITION)
            {
                count += 1;
            }
        }
        let answer = count;
        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
