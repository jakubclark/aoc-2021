#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    row: i32,
    column: i32,
    value: u32,
}

type Row = Vec<Point>;
type Matrix = Vec<Row>;

fn parse_input() -> Matrix {
    let input = include_str!("../input.txt");
    let mut result = Vec::new();

    for (row, line) in input.lines().enumerate() {
        let mut row_vec = Vec::new();
        for (column, c) in line.chars().enumerate() {
            let value = c.to_digit(10).unwrap();
            let point = Point {
                row: row as i32,
                column: column as i32,
                value,
            };
            row_vec.push(point);
        }
        result.push(row_vec);
    }

    result
}

trait Neighbors {
    fn get_item(&self, row_index: i32, column_index: i32) -> Option<&Point>;
    fn get_neighbors(&self, row_index: i32, column_index: i32) -> Vec<&Point>;
    fn get_lowpoints(&self) -> Vec<&Point>;
    fn get_basin(&self, row_index: i32, column_index: i32) -> Option<&Point>;
    fn get_lowest_neighbor(&self, row_index: i32, column_index: i32) -> Option<&Point>;
}

impl Neighbors for Matrix {
    fn get_item(&self, row_index: i32, column_index: i32) -> Option<&Point> {
        if row_index < 0 || column_index < 0 {
            None
        } else {
            self.get(row_index as usize)
                .and_then(|row| row.get(column_index as usize))
        }
    }

    fn get_neighbors(&self, row_index: i32, column_index: i32) -> Vec<&Point> {
        let mut result = Vec::new();
        let indices = [
            (row_index - 1, column_index), // Above
            (row_index + 1, column_index), // Below
            (row_index, column_index - 1), // Left
            (row_index, column_index + 1), // Right
        ];
        for (row_index, column_index) in indices {
            if let Some(neighbor) = self.get_item(row_index, column_index) {
                result.push(neighbor);
            }
        }
        result
    }

    fn get_lowpoints(&self) -> Vec<&Point> {
        let mut result = Vec::new();

        for row_index in 0..self.len() {
            for column_index in 0..self[row_index].len() {
                let current = &self[row_index][column_index];

                let row_index = row_index as i32;
                let column_index = column_index as i32;

                let neighbors = self.get_neighbors(row_index, column_index);

                if neighbors.iter().all(|&x| x.value > current.value) {
                    result.push(current);
                }
            }
        }

        result
    }

    fn get_basin(&self, row_index: i32, column_index: i32) -> Option<&Point> {
        let lowpoints = self.get_lowpoints();

        let current = self.get_item(row_index, column_index)?;

        if current.value == 9 {
            return None;
        }

        let neighbor = self.get_lowest_neighbor(current.row as i32, current.column as i32);
        if let Some(neighbor) = neighbor {
            if lowpoints.iter().any(|&x| x == neighbor) {
                Some(neighbor)
            } else {
                self.get_basin(neighbor.row as i32, neighbor.column as i32)
            }
        } else {
            None
        }
    }

    fn get_lowest_neighbor(&self, row_index: i32, column_index: i32) -> Option<&Point> {
        let neighbors = self.get_neighbors(row_index, column_index);
        neighbors.into_iter().min_by(|&a, &b| a.value.cmp(&b.value))
    }
}

fn solve_part1() -> u32 {
    let rows = parse_input();
    let lowpoints = rows.get_lowpoints();
    lowpoints.into_iter().map(|x| x.value + 1).sum()
}

fn solve_part2() -> usize {
    let rows = parse_input();
    let mut mapping = std::collections::HashMap::<&Point, Vec<&Point>>::new();
    for row in &rows {
        for point in row {
            let current = rows
                .get_item(point.row as i32, point.column as i32)
                .unwrap();
            let basin = rows.get_basin(point.row as i32, point.column as i32);
            let basin = match basin {
                Some(basin) => basin,
                None => continue,
            };
            if let Some(entry) = mapping.get_mut(basin) {
                entry.push(current);
            } else {
                mapping.insert(basin, vec![current]);
            }
        }
    }

    let mut basins = Vec::new();
    for (_, basin) in mapping.into_iter() {
        basins.push(basin);
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));

    basins[0].len() * basins[1].len() * basins[2].len()
}

fn main() {
    let part1 = solve_part1();
    println!("Part 1: {}", part1);
    let part2 = solve_part2();
    println!("Part 2: {}", part2);
}
