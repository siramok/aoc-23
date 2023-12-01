use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn part_one() {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut first: char = '0';
                let mut last: char = '0';
                let mut first_found: bool = false;
                for c in ip.chars() {
                    if c.is_ascii_digit() {
                        if !first_found {
                            first_found = true;
                            first = c;
                        }
                        last = c
                    }
                }
                let mut num = String::new();
                num.push(first);
                num.push(last);
                let num: u32 = num.parse().unwrap();
                sum += num;
            }
        }
    }
    println!("{}", sum);
}

fn part_two() {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let mut first: char = '0';
                let mut last: char = '0';
                let mut word: String = String::new();
                let mut first_found: bool = false;
                for c in ip.chars() {
                    if c.is_ascii_digit() {
                        if !first_found {
                            first_found = true;
                            first = c;
                        }
                        last = c;
                        word.clear();
                    } else {
                        word.push(c);
                        println!("{}", word.as_str());
                        let word_to_digit: char = match word.as_str() {
                            "zero" => '0',
                            "one" => '1',
                            "two" => '2',
                            "three" => '3',
                            "four" => '4',
                            "five" => '5',
                            "six" => '6',
                            "seven" => '7',
                            "eight" => '8',
                            "nine" => '9',
                            _ => ' ',
                        };
                        if word_to_digit.is_ascii_digit() {
                            if !first_found {
                                first_found = true;
                                first = word_to_digit;
                            }
                            last = word_to_digit;
                        }
                    }
                }
                let mut num = String::new();
                num.push(first);
                num.push(last);
                let num: u32 = num.parse().unwrap();
                println!("{}", num);
                sum += num;
            }
        }
    }
    println!("{}", sum);
}

fn main() {
    part_one();
    part_two();
}
