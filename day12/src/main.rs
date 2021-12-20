use std::collections::HashMap;

fn parse_input<'a>() -> Visitor<'a> {
    let input = include_str!("../input.txt");
    let mut connections = HashMap::<&'a str, Vec<&'a str>>::new();
    for line in input.lines() {
        let (start, end) = line.split_once("-").unwrap();

        if let Some(x) = connections.get_mut(start) {
            x.push(end)
        } else {
            connections.insert(start, vec![end]);
        }

        if let Some(x) = connections.get_mut(end) {
            x.push(start);
        } else {
            connections.insert(end, vec![start]);
        }
    }
    Visitor {
        connections,
        paths: vec![],
        current_path: vec!["start"],
    }
}

struct Visitor<'a> {
    connections: HashMap<&'a str, Vec<&'a str>>,
    paths: Vec<Vec<&'a str>>,
    current_path: Vec<&'a str>,
}

impl<'a> Visitor<'a> {
    fn visited_small_cave(&self, cave: &str) -> bool {
        cave.chars().all(|c| c.is_lowercase()) && self.current_path.iter().any(|&c| c == cave)
    }

    fn is_small_cave(cave: &str) -> bool {
        cave.chars().all(|c| c.is_lowercase())
    }

    fn traverse(&mut self, cave: &[&'a str]) {
        for &x in cave {
            if self.visited_small_cave(x) {
                continue;
            } else if x == "end" {
                self.current_path.push(x);
                self.paths.push(self.current_path.clone());
                self.current_path.pop();
            } else {
                self.current_path.push(x);
                self.traverse(&self.connections[x].to_owned());
            }
        }
        self.current_path.pop();
    }

    fn can_visit_small_cave_again(&self, point: &str) -> bool {
        if !Self::is_small_cave(point) {
            return true;
        }
        let mut seen = HashMap::new();
        for &c in &self.current_path {
            if !Self::is_small_cave(c) {
                continue;
            }
            let entry = seen.entry(c).or_insert(0);
            *entry += 1;
        }

        if seen.get(point).is_some() {
            for entry in seen.values() {
                if *entry > 1 {
                    return false;
                }
            }
            true
        } else {
            true
        }
    }

    fn traverse_part2(&mut self, cave: &[&'a str]) {
        for &x in cave {
            if x == "start" {
                continue;
            } else if x == "end" {
                self.current_path.push(x);
                self.paths.push(self.current_path.clone());
                self.current_path.pop();
            } else if !self.can_visit_small_cave_again(x) {
                continue;
            } else {
                self.current_path.push(x);
                self.traverse_part2(&self.connections[x].to_owned());
            }
        }
        self.current_path.pop();
    }
}

fn solve_part1() -> u32 {
    let mut visitor = parse_input();
    let next = &visitor.connections["start"].to_owned();
    visitor.traverse(next);

    let mut sum = 0;
    for path in visitor.paths {
        if path.iter().any(|&c| Visitor::is_small_cave(c)) {
            sum += 1;
        }
    }
    sum
}

fn solve_part2() -> u32 {
    let mut visitor = parse_input();
    let next = &visitor.connections["start"].to_owned();
    visitor.traverse_part2(next);
    visitor.paths.len() as u32
}

fn main() {
    let part1 = solve_part1();
    println!("Part 1: {}", part1);
    let part2 = solve_part2();
    println!("Part 2: {}", part2);
}
