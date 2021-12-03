use std::cmp::Ordering;

fn solve_part1(input: &str) -> u32 {
    let first = input.lines().next().unwrap();
    let count = first.len();
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..count {
        let mut total_zeroes = 0;
        let mut total_ones = 0;
        for line in input.lines() {
            let c = &line[i..=i];
            if c == "0" {
                total_zeroes += 1;
            } else if c == "1" {
                total_ones += 1;
            }
        }

        match total_zeroes.cmp(&total_ones) {
            Ordering::Greater => {
                gamma <<= 1;
                epsilon = (epsilon << 1) | 1;
            }
            Ordering::Less => {
                gamma = (gamma << 1) | 1;
                epsilon <<= 1;
            }
            _ => unreachable!(),
        };
    }
    gamma * epsilon
}

fn solve_part2(input: &str) -> u32 {
    let diagnostics: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let number_of_bits = diagnostics[0].len();

    let oxygen_generator_rating = compute_rating(diagnostics.clone(), number_of_bits, '0');
    let carbon_dioxide_scrubber_rating = compute_rating(diagnostics, number_of_bits, '1');
    oxygen_generator_rating * carbon_dioxide_scrubber_rating
}

fn compute_rating(
    mut diagnostics: Vec<Vec<char>>,
    number_of_bits: usize,
    significant_character: char,
) -> u32 {
    for bit_position in 0..number_of_bits {
        if diagnostics.len() == 1 {
            break;
        }
        let mut indices_with_zero = Vec::new();
        let mut indices_with_one = Vec::new();

        for (i, reading) in diagnostics.iter().enumerate() {
            if reading[bit_position] == '0' {
                indices_with_zero.push(i);
            } else if reading[bit_position] == '1' {
                indices_with_one.push(i);
            }
        }

        if indices_with_one.len() >= indices_with_zero.len() {
            // 1 is most common
            diagnostics = diagnostics
                .into_iter()
                .filter(|c| c[bit_position] == significant_character)
                .collect();
        } else {
            // 0 is most common
            diagnostics = diagnostics
                .into_iter()
                .filter(|c| c[bit_position] != significant_character)
                .collect();
        }
    }

    let reading = diagnostics[0].iter().cloned().collect::<String>();
    u32::from_str_radix(reading.as_str(), 2).unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    let part2 = solve_part2(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
