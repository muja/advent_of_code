#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
  input.split_whitespace().map(|i| i.parse::<i32>().unwrap()).fold(0, std::ops::Add::add)
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
  let mut set = vec![0i32];
  let mut current = 0;
  let mut n = 0i32;
  input.split_whitespace().map(|i| i.parse::<i32>().unwrap()).cycle().take_while(|i| {
    n += 1;
    current += i;
    let b = !set.contains(&current);
    set.push(current);
    b
  }).last().unwrap(); // to ensure the iterator is consumed
  println!("{}", n);
  current
}
