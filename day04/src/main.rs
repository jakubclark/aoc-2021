#[derive(Default)]
struct Board {
    rows: Vec<Vec<(u32, bool)>>,
    had_bingo: bool,
}

impl Board {
    fn has_bingo(&self) -> bool {
        let num_rows = self.rows.len();
        let num_cols = self.rows[0].len();
        let mut column = Vec::new();
        for i in 0..num_rows {
            let row = &self.rows[i];
            if row.iter().all(|&(_, marked)| marked) {
                return true;
            }

            for j in 0..num_cols {
                column.push(self.rows[j][i]);
            }
            if column.iter().all(|&(_, marked)| marked) {
                return true;
            }
            column.clear();
        }

        false
    }

    fn mark_number(&mut self, value: u32) {
        for row in &mut self.rows {
            for (val, marked) in row {
                if *val == value {
                    *marked = true;
                }
            }
        }
    }

    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for row in &self.rows {
            for &(val, marked) in row {
                if marked {
                    continue;
                }
                sum += val;
            }
        }
        sum
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = input.lines();
    let draw_order: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    // Skip the empty line
    lines.next();

    let mut boards = Vec::new();

    let mut current_board = Board::default();

    for line in lines {
        if line.is_empty() {
            boards.push(current_board);
            current_board = Board::default();
            continue;
        }
        let row = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .map(|x| (x, false))
            .collect();
        current_board.rows.push(row);
    }
    boards.push(current_board);

    (draw_order, boards)
}

fn solve_part1(input: &str) -> u32 {
    let (draw_order, mut boards) = parse_input(input);
    for num in draw_order {
        for board in boards.iter_mut() {
            board.mark_number(num);
            if board.has_bingo() {
                let sum = board.sum_unmarked();
                return num * sum;
            }
        }
    }
    0
}

fn solve_part2(input: &str) -> u32 {
    let (draw_order, mut boards) = parse_input(input);
    let mut last_drawn_bingo_number = 0;
    let mut last_bingo_board_index = 0;
    for num in draw_order {
        if boards.iter().all(|board| board.had_bingo) {
            break;
        }
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark_number(num);
            if board.has_bingo() && !board.had_bingo {
                board.had_bingo = true;
                last_drawn_bingo_number = num;
                last_bingo_board_index = i;
            }
        }
    }
    let bingo_board = &boards[last_bingo_board_index];
    let sum = bingo_board.sum_unmarked();
    sum * last_drawn_bingo_number
}

fn main() {
    let input = include_str!("../input.txt");
    let part1 = solve_part1(input);
    println!("Part 1: {}", part1);
    let part2 = solve_part2(input);
    println!("Part 2: {}", part2);
}
