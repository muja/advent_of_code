use std::collections::HashSet;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    input
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .fold(0, std::ops::Add::add)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut set = HashSet::new();
    set.insert(0);
    let mut current = 0;
    input
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .cycle()
        .take_while(|i| {
            current += i;
            set.insert(current)
        })
        .last()
        .unwrap(); // to ensure the iterator is consumed
    current
}
