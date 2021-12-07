fn parse_input(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

fn get_total_fuel_p1(positions: &[i32], target_position: i32) -> i32 {
    positions
        .iter()
        .map(|&start_position| (target_position - start_position).abs())
        .sum()
}

fn get_total_fuel_p2(positions: &[i32], target_position: i32) -> i32 {
    positions
        .iter()
        .map(|&start_position| {
            let distance = (target_position - start_position).abs();
            distance * (distance + 1) / 2
        })
        .sum()
}

fn solve_part1(input: &str) -> i32 {
    let horizontal_positions = parse_input(input);
    horizontal_positions
        .iter()
        .map(|&target_position| get_total_fuel_p1(horizontal_positions.as_slice(), target_position))
        .min()
        .unwrap()
}

fn solve_part2(input: &str) -> i32 {
    let horizontal_positions = parse_input(input);
    let max = *horizontal_positions.iter().max().unwrap();

    (0..=max)
        .map(|target_position| get_total_fuel_p2(horizontal_positions.as_slice(), target_position))
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1: {}", part1);
    let part2 = solve_part2(input);
    println!("Part 2: {}", part2);
}
