use std::collections::HashMap;

type Element = char;
type Pair = (Element, Element);

#[derive(Debug)]
struct Polymizer {
    formulas: HashMap<Pair, Element>,
    elements: HashMap<Element, usize>,
    pairs: HashMap<Pair, usize>,
}

impl Polymizer {
    fn step(&mut self) {
        for (pair, count) in self.pairs.to_owned() {
            let new_element = *self.formulas.get(&pair).unwrap();
            *self.elements.entry(new_element.to_owned()).or_insert(0) += count;
            *self.pairs.entry(pair.to_owned()).or_insert(0) -= count;
            *self.pairs.entry((pair.0, new_element)).or_insert(0) += count;
            *self.pairs.entry((new_element, pair.1)).or_insert(0) += count;
        }
    }
}

fn parse_input() -> Polymizer {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();

    let initial_formula = String::from(lines.next().unwrap());
    let _ = lines.next();

    let mut formulas = HashMap::new();
    for line in lines {
        let (input, output) = line.split_once(" -> ").unwrap();
        let mut input = input.chars();
        let input = (input.next().unwrap(), input.next().unwrap());
        let output = output.chars().next().unwrap();
        formulas.insert(input, output);
    }

    let mut pairs = HashMap::new();
    let mut elements = HashMap::new();

    for i in 0..initial_formula.len() - 1 {
        let mut chars = initial_formula.chars();
        let element1 = chars.nth(i).unwrap();
        let element2 = chars.next().unwrap();
        let pair = (element1, element2);
        *pairs.entry(pair).or_insert(0) += 1;
        *elements.entry(element1).or_insert(0) += 1;
    }

    let element = initial_formula.chars().last().unwrap();
    *elements.entry(element).or_insert(0) += 1;

    Polymizer {
        formulas,
        elements,
        pairs,
    }
}

fn solve(num_steps: u8) -> usize {
    let mut polymizer = parse_input();
    for _ in 0..num_steps {
        polymizer.step();
    }

    let max = *polymizer.elements.values().max().unwrap();
    let min = *polymizer.elements.values().min().unwrap();
    max - min
}

fn solve_part1() -> usize {
    solve(10)
}

fn solve_part2() -> usize {
    solve(40)
}

fn main() {
    let part1 = solve_part1();
    println!("Part 1: {}", part1);
    let part2 = solve_part2();
    println!("Part 2: {}", part2);
}
