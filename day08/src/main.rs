use std::collections::HashSet;

#[derive(Debug)]
struct SegmentDisplay(HashSet<char>);

impl From<&str> for SegmentDisplay {
    fn from(s: &str) -> SegmentDisplay {
        SegmentDisplay(s.chars().collect())
    }
}

#[derive(Debug)]
struct Entry {
    initial_displays: Vec<SegmentDisplay>,
    final_displays: Vec<SegmentDisplay>,
}

impl From<&str> for Entry {
    fn from(s: &str) -> Entry {
        let mut split = s.split('|');
        let initial_displays = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(From::from)
            .collect();
        let final_displays = split
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(From::from)
            .collect();

        Entry {
            initial_displays,
            final_displays,
        }
    }
}

impl SegmentDisplay {
    fn num_segments(&self) -> usize {
        self.0.len()
    }
}

fn parse_input(input: &str) -> Vec<Entry> {
    input.lines().map(From::from).collect()
}

fn solve_part1(input: &str) -> usize {
    let entries = parse_input(input);
    entries
        .into_iter()
        .map(|entry| {
            entry
                .final_displays
                .iter()
                .filter(|&x| {
                    let segment_count = x.num_segments();
                    segment_count == 2
                        || segment_count == 3
                        || segment_count == 4
                        || segment_count == 7
                })
                .count()
        })
        .sum()
}

fn solve_part2(_: &str) -> u32 {
    0
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1: {}", part1);
    let part2 = solve_part2(input);
    println!("Part 2: {}", part2);
}
