use std::collections::HashSet;

pub fn soln(input: &str) -> u32 {
    let tokens: Vec<&str> = input.split("\n").collect();
    let rows: usize = tokens.len();
    let cols: usize = tokens[0].len();
    let input: Vec<char> = tokens.join("").chars().collect();
    let special_symbols: HashSet<char> =
        HashSet::from(['*', '#', '+', '$', '@', '&', '/', '%', '=', '-']);
    let mut sum: u32 = 0;
    for i in 0..rows {
        let mut is_part_number: bool = false;
        let mut num_as_char_vec: Vec<char> = Vec::new();
        for j in 0..cols {
            let index: usize = i * cols + j;
            let c: char = input[index];
            if c.is_digit(10) {
                println!("Before -> index = {}, c = {}", index, c);
                num_as_char_vec.push(c);

                if is_part_number {
                    println!("skipping");
                    continue;
                }
                let mut up_left: char = ' ';
                let mut up: char = ' ';
                let mut up_right: char = ' ';
                let mut left: char = ' ';
                let mut right: char = ' ';
                let mut down_left: char = ' ';
                let mut down: char = ' ';
                let mut down_right: char = ' ';
                if index == 0 {
                    // Top left corner
                    println!("TLC -> index = {}, c = {}", index, c);
                    right = input[index + 1];
                    down = input[index + cols];
                    down_right = input[index + cols + 1];
                } else if index == (cols - 1) {
                    // Top right corner
                    println!("TRC -> index = {}, c = {}", index, c);
                    left = input[index - 1];
                    down_left = input[index + cols - 1];
                    down = input[index + cols];
                } else if index == (rows - 1) * cols {
                    // Bottom left corner
                    println!("BLC -> index = {}, c = {}", index, c);
                    up = input[index - cols];
                    up_right = input[index - cols + 1];
                    right = input[index + 1];
                } else if index == rows * cols - 1 {
                    // Bottom right corner
                    println!("BRC -> index = {}, c = {}", index, c);
                    up_left = input[index - cols - 1];
                    up = input[index - cols];
                    left = input[index - 1];
                } else if index < cols {
                    // Top boundary
                    println!("TB -> index = {}, c = {}", index, c);
                    left = input[index - 1];
                    right = input[index + 1];
                    down_left = input[index + cols - 1];
                    down = input[index + cols];
                    down_right = input[index + cols + 1];
                } else if index % cols == 0 {
                    // Left boundary
                    println!("LB -> index = {}, c = {}", index, c);
                    up = input[index - cols];
                    up_right = input[index - cols + 1];
                    right = input[index + 1];
                    down = input[index + cols];
                    down_right = input[index + cols + 1];
                } else if index % cols == cols - 1 {
                    // Right boundary
                    println!("RB -> index = {}, c = {}", index, c);
                    up_left = input[index - cols - 1];
                    up = input[index - cols];
                    left = input[index - 1];
                    down_left = input[index + cols - 1];
                    down = input[index + cols];
                } else if index >= (rows - 1) * cols {
                    // Bottom boundary
                    println!("BB -> index = {}, c = {}", index, c);
                    up_left = input[index - cols - 1];
                    up = input[index - cols];
                    up_right = input[index - cols + 1];
                    left = input[index - 1];
                    right = input[index + 1];
                } else {
                    // Not a boundary
                    println!("NB -> index = {}, c = {}", index, c);
                    up_left = input[index - cols - 1];
                    up = input[index - cols];
                    up_right = input[index - cols + 1];
                    left = input[index - 1];
                    right = input[index + 1];
                    down_left = input[index + cols - 1];
                    down = input[index + cols];
                    down_right = input[index + cols + 1];
                }

                is_part_number |= [
                    up_left, up, up_right, left, right, down_left, down, down_right,
                ]
                .iter()
                .any(|&symbol| special_symbols.contains(&symbol));
                println!("match found");
            } else if !num_as_char_vec.is_empty() {
                if is_part_number {
                    // println!("{:?}", num_as_char_vec);
                    let num: String = num_as_char_vec.clone().into_iter().collect();
                    let num: u32 = num.parse::<u32>().unwrap();
                    sum += num;
                    println!("adding {}", num);
                } else {
                    let num: String = num_as_char_vec.clone().into_iter().collect();
                    let num: u32 = num.parse::<u32>().unwrap();
                    println!("not adding {}", num);
                }
                is_part_number = false;
                num_as_char_vec.clear();
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let result = soln(input);
        assert_eq!(result, 4361);
    }
}
