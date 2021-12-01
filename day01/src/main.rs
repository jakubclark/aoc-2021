fn solve_part1(nums: &[u16]) -> u16 {
    compute_consecutive_increasing_windows(nums, 1)
}

fn solve_part2(nums: &[u16]) -> u16 {
    compute_consecutive_increasing_windows(nums, 3)
}

fn compute_consecutive_increasing_windows(nums: &[u16], window_size: usize) -> u16 {
    let mut count = 0;
    for i in window_size..nums.len() {
        let previous_window: u16 = nums[i - window_size..=i - 1].iter().sum();
        let current_window: u16 = nums[i - window_size + 1..=i].iter().sum();
        if current_window > previous_window {
            count += 1
        }
    }
    count
}

fn main() {
    let input = include_str!("../input.txt");
    let nums: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();
    let part1 = solve_part1(&nums);
    println!("Part 1 solution: {}", part1);
    let part2 = solve_part2(&nums);
    println!("Part 2 solution: {}", part2);
}
