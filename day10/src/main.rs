fn parse_input<'a>() -> Vec<&'a str> {
    let input = include_str!("../input.txt");
    input.lines().collect()
}

const BRACKETS: &[(char, char)] = &[('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

fn is_opening(c: char) -> bool {
    BRACKETS.iter().any(|(x, _)| *x == c)
}

fn is_closing(c: char) -> bool {
    BRACKETS.iter().any(|(_, x)| *x == c)
}

// Returns the first illegal character, if the line is corrupted
fn is_corrupted(line: &str) -> Option<char> {
    let mut stack = Vec::new();

    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else if is_closing(c) {
            let actual_opening = match stack.pop() {
                Some(opening) => opening,
                None => continue,
            };
            let expected_opening = match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                '>' => '<',
                _ => unreachable!(),
            };

            if expected_opening != actual_opening {
                return Some(c);
            }
        }
    }
    None
}

// Returns the missing closing brackets, for the line
fn complete(line: &str) -> String {
    let mut stack = Vec::new();
    let mut to_append = String::new();
    for c in line.chars() {
        if is_opening(c) {
            stack.push(c);
        } else if is_closing(c) {
            let _ = stack.pop();
        }
    }

    while let Some(opening) = stack.pop() {
        let closing = match opening {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unreachable!(),
        };
        to_append.push(closing);
    }
    to_append
}

fn solve_part1() -> usize {
    let lines = parse_input();
    let mut sum = 0;
    for line in lines {
        if let Some(illegal) = is_corrupted(line) {
            sum += match illegal {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            };
        }
    }
    sum
}

fn solve_part2() -> usize {
    let lines = parse_input();
    let mut sums = Vec::new();
    for line in lines {
        if is_corrupted(line).is_some() {
            continue;
        }
        let missing = complete(line);

        let mut sum = 0;

        for c in missing.chars() {
            sum *= 5;
            sum += match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => unreachable!(),
            };
        }

        sums.push(sum);
    }
    sums.sort();
    sums.reverse();
    sums[sums.len() / 2]
}

fn main() {
    let part1 = solve_part1();
    println!("Part 1: {}", part1);
    let part2 = solve_part2();
    println!("Part 2: {}", part2);
}
