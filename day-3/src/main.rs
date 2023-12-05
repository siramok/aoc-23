mod part1;
mod part2;

fn main() {
    let input = include_str!("./input.txt");
    println!("Part 1 solution: {}", part1::soln(input));
    println!("Part 2 solution: {}", part2::soln(input));
}
