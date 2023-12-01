use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn word_to_digit(word: String) -> i32 {
    match word.as_str() {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => -1,
    }
}

fn part_one(filename: &str) {
    let radix: u32 = 10;
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let line_as_vec: Vec<char> = ip.chars().collect();
                let mut first: i32 = -1;
                let mut last: i32 = -1;
                for c in line_as_vec.iter() {
                    if c.is_ascii_digit() {
                        if first == -1 {
                            first = c.to_digit(radix).unwrap() as i32;
                        }
                        last = c.to_digit(radix).unwrap() as i32;
                    }
                }
                let num: i32 = first * 10 + last;
                sum += num;
            }
        }
    }
    println!("{}", sum);
}

fn part_two(filename: &str) {
    let radix: u32 = 10;
    let mut sum: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                let line_as_vec: Vec<char> = ip.chars().collect();
                let mut first: i32 = -1;
                let mut last: i32 = -1;
                for (i, c) in line_as_vec.iter().enumerate() {
                    let mut digit: i32 = -1;
                    if c.is_ascii_digit() {
                        digit = c.to_digit(radix).unwrap() as i32;
                    } else {
                        // println!("i = {}, len = {}", i, line_as_vec.len());
                        if i + 2 < line_as_vec.len() {
                            digit = word_to_digit(
                                line_as_vec[i..i + 3].iter().cloned().collect::<String>(),
                            );
                        }
                        if digit == -1 && i + 3 < line_as_vec.len() {
                            digit = word_to_digit(
                                line_as_vec[i..i + 4].iter().cloned().collect::<String>(),
                            );
                        }
                        if digit == -1 && i + 4 < line_as_vec.len() {
                            digit = word_to_digit(
                                line_as_vec[i..i + 5].iter().cloned().collect::<String>(),
                            );
                        }
                    }
                    if digit != -1 {
                        if first == -1 {
                            first = digit;
                        }
                        last = digit;
                    }
                }
                let num: i32 = first * 10 + last;
                sum += num;
            }
        }
    }
    println!("{}", sum);
}

fn main() {
    let filename: &str = "input.txt";
    part_one(&filename);
    part_two(&filename);
}
