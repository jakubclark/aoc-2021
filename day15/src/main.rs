use priority_queue::DoublePriorityQueue;
use std::collections::HashMap;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
struct Node {
    x: usize,
    y: usize,
    risk_level: usize,
}

#[derive(Debug)]
struct Matrix {
    q: DoublePriorityQueue<Node, usize>,
    dist: HashMap<Node, usize>,
    prev: HashMap<Node, Node>,
    grid: Vec<Vec<Node>>,
    start: Node,
    target: Node,
}

fn parse_input() -> Matrix {
    let input = include_str!("../input.txt");

    let mut dist = HashMap::new();
    let prev = HashMap::new();
    let mut grid = Vec::new();
    let mut q = DoublePriorityQueue::new();

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let risk_level = c.to_digit(10).unwrap() as usize;
            let node = Node { x, y, risk_level };
            dist.insert(node, usize::MAX);
            q.push(node, usize::MAX);
            row.push(node);
        }
        grid.push(row);
    }

    let start = grid[0][0];
    dist.insert(start, 0);

    let target = *grid.last().unwrap().last().unwrap();

    Matrix {
        q,
        dist,
        prev,
        grid,
        start,
        target,
    }
}

impl Matrix {
    fn compute_shortest_paths(&mut self) {
        while let Some((u, _)) = self.q.pop_min() {
            for v in self.get_neighbors(u) {
                let x = *self.dist.get(&u).unwrap();
                if x == usize::MAX {
                    continue;
                }
                let alt = *self.dist.get(&u).unwrap() + u.risk_level;
                if alt < *self.dist.get(&v).unwrap() {
                    self.dist.insert(v, alt);
                    self.prev.insert(v, u);
                    self.q.push_decrease(v, alt);
                }
            }
        }
    }

    fn find_shortest_path_to_target(&self) -> Vec<&Node> {
        let mut path = Vec::new();
        let mut u = &self.target;
        if self.prev.get(u).is_some() || *u == self.start {
            loop {
                path.push(u);
                if let Some(x) = self.prev.get(u) {
                    u = x;
                } else {
                    break;
                }
            }
        }
        path
    }

    fn get_neighbors(&self, node: Node) -> Vec<Node> {
        let mut result = Vec::new();
        let (x, y) = (node.x as i32, node.y as i32);
        let indices = [
            (x, y - 1), // Above
            (x + 1, y), // Right
            (x, y + 1), // Below
            (x - 1, y), // Left
        ];
        for (x, y) in indices {
            if x < 0 || y < 0 {
                continue;
            }
            if let Some(node) = self
                .grid
                .get(y as usize)
                .and_then(|row| row.get(x as usize))
            {
                result.push(*node);
            }
        }

        result
    }

    fn expand(self, size: usize) -> Matrix {
        let original_width = self.grid[0].len();
        let new_width = original_width * size;
        let original_height = self.grid.len();
        let new_height = original_height * size;

        let mut q = DoublePriorityQueue::new();
        let mut dist = HashMap::new();
        let prev = HashMap::new();

        let mut grid = vec![vec![Node::default(); new_width]; new_height];

        for (node, _) in self.q {
            for x_multi in 0..size {
                for y_multi in 0..size {
                    let x = node.x + (x_multi * original_width);
                    let y = node.y + (y_multi * original_height);
                    let risk_level = node.risk_level + x_multi + y_multi;
                    let risk_level = if risk_level >= 10 {
                        risk_level % 9
                    } else {
                        risk_level
                    };
                    let node = Node { x, y, risk_level };
                    dist.insert(node, usize::MAX);
                    q.push(node, usize::MAX);
                    grid[y][x] = node;
                }
            }
        }

        let start = grid[0][0];
        dist.insert(start, 0);

        let target = grid[new_height - 1][new_width - 1];

        Matrix {
            q,
            dist,
            prev,
            grid,
            start,
            target,
        }
    }
}

fn solve_part1() -> usize {
    let mut matrix = parse_input();
    matrix.compute_shortest_paths();
    let mut path = matrix.find_shortest_path_to_target();
    path.pop();
    path.iter().map(|&n| n.risk_level).sum()
}

fn solve_part2() -> usize {
    let mut matrix = parse_input().expand(5);
    matrix.compute_shortest_paths();
    let mut path = matrix.find_shortest_path_to_target();
    path.pop();
    path.iter().map(|&n| n.risk_level).sum()
}

fn main() {
    let part1 = solve_part1();
    println!("Part 1: {}", part1);
    let part2 = solve_part2();
    println!("Part 2: {}", part2);
}
