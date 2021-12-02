fn solve_part1(input: &[(&str, i32)]) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    for &(direction, distance) in input {
        match direction {
            "forward" => horizontal_position += distance,
            "up" => depth -= distance,
            "down" => depth += distance,
            _ => unreachable!(),
        }
    }
    depth * horizontal_position
}

fn solve_part2(input: &[(&str, i32)]) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for &(direction, distance) in input {
        match direction {
            "forward" => {
                horizontal_position += distance;
                depth += aim * distance;
            }
            "up" => aim -= distance,
            "down" => aim += distance,
            _ => unreachable!(),
        }
    }
    depth * horizontal_position
}

fn main() {
    let input = include_str!("../input.txt");
    let instructions: Vec<(&str, i32)> = input
        .lines()
        .map(|l| {
            let mut split = l.split_whitespace();
            let direction = split.next().unwrap();
            let distance = split.next().unwrap().parse().unwrap();
            (direction, distance)
        })
        .collect();
    let part1 = solve_part1(&instructions);
    println!("Part 1: {}", part1);
    let part2 = solve_part2(&instructions);
    println!("Part 2: {}", part2);
}
