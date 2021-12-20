use std::{collections::HashSet, fmt::Debug};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Dot {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Fold {
    axis: Axis,
    index: usize,
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

struct Matrix {
    dots: HashSet<Dot>,
}

impl Matrix {
    fn print(&self) {
        let max_x = self.dots.iter().map(|dot| dot.x).max().unwrap() + 1;
        let max_y = self.dots.iter().map(|dot| dot.y).max().unwrap() + 1;
        let mut matrix = vec![vec![false; max_x]; max_y];
        for dot in self.dots.iter() {
            matrix[dot.y][dot.x] = true;
        }
        for row in matrix {
            for value in row {
                let c = if value { '#' } else { '.' };
                print!("{}", c);
            }
            println!();
        }
    }

    fn apply_fold(&mut self, fold: Fold) {
        match fold.axis {
            Axis::X => {
                println!("Applying a vertical fold: {:?}", fold);
                self.apply_vertical_fold(fold);
            }
            Axis::Y => {
                println!("Applying a horizontal fold: {:?}", fold);
                self.apply_horizontal_fold(fold);
            }
        }
    }

    fn apply_vertical_fold(&mut self, fold: Fold) {
        let mut result = HashSet::new();
        for dot in self.dots.iter() {
            if dot.x > fold.index {
                let distance = dot.x - fold.index;
                let x = fold.index - distance;
                let dot = Dot { x, y: dot.y };
                result.insert(dot);
            } else {
                result.insert(dot.to_owned());
            }
        }
        self.dots = result;
    }

    fn apply_horizontal_fold(&mut self, fold: Fold) {
        let mut result = HashSet::new();
        for dot in self.dots.iter() {
            if dot.y > fold.index {
                let distance = dot.y - fold.index;
                let y = fold.index - distance;
                let dot = Dot { x: dot.x, y };
                result.insert(dot);
            } else {
                result.insert(dot.to_owned());
            }
        }
        self.dots = result;
    }
}

fn parse_input() -> (Matrix, Vec<Fold>) {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();

    let mut dots = HashSet::new();
    let mut folds = Vec::new();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(",").unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
        let dot = Dot { x, y };
        dots.insert(dot);
    }

    while let Some(line) = lines.next() {
        let axis = match &line[11..=11] {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => unreachable!(),
        };
        let index = line[13..].parse().unwrap();
        let fold = Fold { axis, index };
        folds.push(fold)
    }

    let matrix = Matrix { dots };

    (matrix, folds)
}

fn solve_part1() -> usize {
    let (mut matrix, mut folds) = parse_input();

    let fold = folds.remove(0);
    matrix.apply_fold(fold);

    matrix.dots.len()
}

fn solve_part2() {
    let (mut matrix, mut folds) = parse_input();
    while folds.len() > 0 {
        let fold = folds.remove(0);
        matrix.apply_fold(fold);
    }
    matrix.print();
}

fn main() {
    let part1 = solve_part1();
    println!("Part 1: {}", part1);
    solve_part2();
}
