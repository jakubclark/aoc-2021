fn parse_input(input: &str) -> [u128; 9] {
    input
        .split(',')
        .map(|s| s.parse().unwrap())
        .fold([0; 9], |mut acc, x: usize| {
            acc[x] += 1;
            acc
        })
}

fn step(d: [u128; 9]) -> [u128; 9] {
    [d[1], d[2], d[3], d[4], d[5], d[6], d[7] + d[0], d[8], d[0]]
}

fn solve(input: &str, num_days: u16) -> u128 {
    let mut timers = parse_input(input);
    for _ in 1..=num_days {
        timers = step(timers);
    }
    timers.iter().sum()
}

fn solve_part1(input: &str) -> u128 {
    solve(input, 80)
}

fn solve_part2(input: &str) -> u128 {
    solve(input, 256)
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1: {}", part1);
    let part2 = solve_part2(input);
    println!("Part 2: {}", part2);
}
