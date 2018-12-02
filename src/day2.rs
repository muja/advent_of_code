use std::collections::HashMap;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
  let mut twos = 0i32;
  let mut threes = 0i32;
  for line in input.lines() {
    let mut map: HashMap<char, i32> = HashMap::new();
    for c in line.chars() {
      if let Some(old) = map.get_mut(&c) {
        *old += 1;
      } else {
        map.insert(c, 1);
      }
    }
    if let Some(_) = map.values().filter(|e| **e == 2).last() {
      twos += 1;
    }
    if let Some(_) = map.values().filter(|e| **e == 3).last() {
      threes += 1;
    }
  }
  twos * threes
}
