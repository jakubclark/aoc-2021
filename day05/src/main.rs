type Row = Vec<u8>;

#[derive(Debug, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Debug, Hash)]
struct Line {
    start: Coordinates,
    end: Coordinates,
}

impl Line {
    fn segments(&self, include_diagonal: bool) -> Vec<(usize, usize)> {
        let mut segments = Vec::new();
        if self.start.x == self.end.x {
            let (start, end) = if self.start.y > self.end.y {
                (self.end.y, self.start.y)
            } else {
                (self.start.y, self.end.y)
            };
            for y in start..=end {
                segments.push((self.start.x, y));
            }
        } else if self.start.y == self.end.y {
            let (start, end) = if self.start.x > self.end.x {
                (self.end.x, self.start.x)
            } else {
                (self.start.x, self.end.x)
            };
            for x in start..=end {
                segments.push((x, self.start.y));
            }
        } else if include_diagonal {
            let mut x = self.start.x;
            let mut y = self.start.y;

            let increasing_x = self.end.x > self.start.x;
            let increasing_y = self.end.y > self.start.y;

            loop {
                segments.push((x, y));
                if x == self.end.x || y == self.end.y {
                    break;
                }
                if increasing_x {
                    x += 1;
                } else {
                    x -= 1;
                }
                if increasing_y {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
        segments
    }
}

struct Board {
    rows: Vec<Row>,
    vent_paths: Vec<Line>,
}

fn parse_input(input: &str) -> Board {
    let parse_coordinates = |part: &str| {
        let mut split = part.trim().split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        Coordinates { x, y }
    };
    let parse_line = |line: &str| {
        let mut split = line.split("->");
        let start = parse_coordinates(split.next().unwrap());
        let end = parse_coordinates(split.next().unwrap());
        Line { start, end }
    };


    let vent_paths: Vec<Line> = input.lines().map(parse_line).collect();

    let x1 = vent_paths.iter().map(|line| line.end.x).max().unwrap();
    let x2 = vent_paths.iter().map(|line| line.start.x).max().unwrap();
    let max_x = std::cmp::max(x1, x2) as usize;

    let y1 = vent_paths.iter().map(|line| line.end.y).max().unwrap();
    let y2 = vent_paths.iter().map(|line| line.start.y).max().unwrap();
    let max_y = std::cmp::max(y1, y2) as usize;

    let number_of_columns = max_x + 1 as usize;
    let number_of_rows = max_y + 1;

    let mut rows = Vec::new();
    for _ in 0..number_of_rows {
        let row = vec![0; number_of_columns];
        rows.push(row);
    }

    Board { rows, vent_paths }
}

fn solve(input: &str, include_diagonal: bool) -> usize {
    let mut board = parse_input(input);
    let mut num_overlaps = 0;

    for line in &board.vent_paths {
        for segment in line.segments(include_diagonal) {
            let x = segment.0 as usize;
            let y = segment.1 as usize;
            board.rows[y][x] += 1;
        }
    }

    for row in &board.rows {
        for &value in row {
            if value > 1 {
                num_overlaps += 1;
            }
        }
    }
    num_overlaps
}

fn solve_part1(input: &str) -> usize {
    solve(input, false)
}

fn solve_part2(input: &str) -> usize {
    solve(input, true)
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1: {}", part1);
    let part2 = solve_part2(input);
    println!("Part 2: {}", part2);
}
