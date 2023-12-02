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

fn clean_and_tokenize(s: &mut String) -> Vec<Vec<&str>> {
    s.replace_range(..s.find(":").unwrap() + 1, "");
    let tokens: Vec<&str> = s.split(";").collect();
    let mut token_vec = Vec::<Vec<&str>>::new();
    for token in tokens.iter() {
        let temp: Vec<&str> = token.split(",").collect();
        token_vec.push(temp.iter().map(|s| &s[1..]).collect());
    }
    token_vec
}

fn get_color_maxes(token_vec: &Vec<Vec<&str>>) -> [u32; 3] {
    let mut maxes: [u32; 3] = [0, 0, 0];
    for tokens in token_vec.iter() {
        for token in tokens.iter() {
            let pair: Vec<&str> = token.split(" ").collect();
            match pair[1] {
                "red" => {
                    let count = pair[0].parse().unwrap();
                    if count > maxes[0] {
                        maxes[0] = count;
                    }
                }
                "green" => {
                    let count = pair[0].parse().unwrap();
                    if count > maxes[1] {
                        maxes[1] = count;
                    }
                }
                "blue" => {
                    let count = pair[0].parse().unwrap();
                    if count > maxes[2] {
                        maxes[2] = count;
                    }
                }
                _ => (),
            }
        }
    }
    maxes
}

fn part_one(filename: &str) {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for (i, line) in lines.enumerate() {
            if let Ok(mut s) = line {
                let token_vec = clean_and_tokenize(&mut s);
                let maxes = get_color_maxes(&token_vec);
                if maxes[0] <= 12 && maxes[1] <= 13 && maxes[2] <= 14 {
                    sum += 1 + i as u32;
                }
            }
        }
    }
    println!("{}", sum);
}

fn part_two(filename: &str) {
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(mut s) = line {
                let token_vec = clean_and_tokenize(&mut s);
                let maxes = find_color_maxes(&token_vec);
                sum += maxes[0] * maxes[1] * maxes[2];
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
