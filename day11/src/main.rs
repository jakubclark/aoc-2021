type Row = Vec<Octopus>;

#[derive(Debug, Clone)]
struct Position {
    row_index: usize,
    column_index: usize,
}

#[derive(Debug)]
struct Octopus {
    energy_level: u8,
    position: Position,
    already_flashed: bool,
}

impl Octopus {
    // If possible, the octopus flashes and returns true
    fn flash(&mut self) -> bool {
        if self.energy_level <= 9 || self.already_flashed {
            false
        } else {
            self.already_flashed = true;
            true
        }
    }
}
struct Matrix(Vec<Row>);

impl Matrix {
    fn get_neighbor_indices(&self, position: Position) -> Vec<Position> {
        let row_index = position.row_index as i32;
        let column_index = position.column_index as i32;
        let indices = [
            (row_index - 1, column_index - 1), // Top-left
            (row_index - 1, column_index),     // Top
            (row_index - 1, column_index + 1), // Top-right
            (row_index, column_index + 1),     // Right
            (row_index + 1, column_index + 1), // Bottom-right
            (row_index + 1, column_index),     // Bottom
            (row_index + 1, column_index - 1), // Bottom-left
            (row_index, column_index - 1),     // Left
        ];

        let number_of_rows = self.0.len() as i32;
        let number_of_columns = self.0[0].len() as i32;

        indices
            .into_iter()
            .filter_map(|(row_index, column_index)| {
                // Only return legal positions
                if row_index < 0
                    || row_index >= number_of_rows
                    || column_index < 0
                    || column_index >= number_of_columns
                {
                    None
                } else {
                    Some(Position {
                        row_index: row_index as usize,
                        column_index: column_index as usize,
                    })
                }
            })
            .collect()
    }

    fn get_octopus_mut(&mut self, position: Position) -> Option<&mut Octopus> {
        self.0
            .get_mut(position.row_index)
            .and_then(|row| row.get_mut(position.column_index))
    }

    fn step(&mut self) -> u32 {
        self.phase1();
        let flashes = self.phase2();
        self.phase3();
        flashes
    }

    /// First, the energy level of each octopus increases by 1.
    fn phase1(&mut self) {
        for row in self.0.iter_mut() {
            for octopus in row.iter_mut() {
                octopus.energy_level += 1;
                octopus.already_flashed = false;
            }
        }
    }

    /// Then, any octopus with an energy level greater than 9 flashes.
    /// This increases the energy level of all adjacent octopuses by 1,
    /// including octopuses that are diagonally adjacent.
    /// If this causes an octopus to have an energy level greater than 9, it also flashes.
    /// This process continues as long as new octopuses
    /// keep having their energy level increased beyond 9.
    /// (An octopus can only flash at most once per step.)
    fn phase2(&mut self) -> u32 {
        let mut sum = 0;
        loop {
            let mut any_flashed = false;
            for row_index in 0..self.0.len() {
                for column_index in 0..self.0[row_index].len() {

                    let octopus = &mut self.0[row_index][column_index];
                    if octopus.energy_level <= 9 || octopus.already_flashed {
                        continue;
                    }

                    // The octopus flashed!
                    any_flashed = true;

                    let position = octopus.position.clone();
                    sum += self.process_octopous(position);
                }
            }

            if !any_flashed {
                break;
            }
        }
        sum
    }

    /// Finally, any octopus that flashed during this step
    /// has its energy level set to 0, as it used all of its energy to flash.
    fn phase3(&mut self) {
        for row in self.0.iter_mut() {
            for octopus in row.iter_mut() {
                if octopus.already_flashed {
                    octopus.energy_level = 0;
                }
            }
        }
    }

    /// Process all the neighbors, for the octopus at the given location
    /// Returns the total number of flashes, caused by the flash of the current octopus
    fn process_neighbors(&mut self, position: Position) -> u32 {
        let mut sum = 0;

        // Increase energy level of all neighbors by 1
        let neighbor_indices = self.get_neighbor_indices(position);
        for neighbor_position in &neighbor_indices {
            let neighbor = self.get_octopus_mut(neighbor_position.to_owned()).unwrap();
            neighbor.energy_level += 1;
        }

        // Process all the neighbor octopuses
        for neighbor_position in &neighbor_indices {
            sum += self.process_octopous(neighbor_position.to_owned())
        }

        sum
    }

    fn process_octopous(&mut self, position: Position) -> u32 {
        let mut sum = 0;

        if let Some(octopus) = self.get_octopus_mut(position.to_owned()) {
            if !octopus.flash() {
                return 0;
            }

            sum += 1;
            sum += self.process_neighbors(position);
        }

        sum
    }
}

fn parse_input() -> Matrix {
    let input = include_str!("../input.txt");

    let mut result = Vec::new();

    for (row_index, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (column_index, c) in line.chars().enumerate() {
            let energy_level = c.to_digit(10).unwrap() as u8;

            let position = Position {
                row_index,
                column_index,
            };

            let octopus = Octopus {
                energy_level,
                position,
                already_flashed: false,
            };
            row.push(octopus);
        }
        result.push(row);
    }
    Matrix(result)
}

fn solve_part1() -> u32 {
    let mut matrix = parse_input();
    let mut num_flashes = 0;
    for _ in 1..=100 {
        num_flashes += matrix.step();
    }
    num_flashes
}

fn solve_part2() -> u32 {
    let mut matrix = parse_input();
    let num_octopuses = (matrix.0.len() * matrix.0[0].len()) as u32;
    let mut step = 0;
    loop {
        step += 1;
        let num_flashes = matrix.step();
        if num_flashes == num_octopuses {
            return step;
        }
    }
}

fn main() {
    let part1 = solve_part1();
    println!("Part 1: {}", part1);
    let part2 = solve_part2();
    println!("Part 2: {}", part2);
}
