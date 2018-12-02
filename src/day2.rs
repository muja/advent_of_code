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
    if map.values().any(|&e| e == 2) {
      twos += 1;
    }
    if map.values().any(|&e| e == 3) {
      threes += 1;
    }
  }
  twos * threes
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> String {
  if let Some((s1, _, id)) = find_similar_ids(input) {
    return s1.chars().enumerate().filter(|&(i, _)| i != id).map(|(_, c)| c).collect::<String>();
  }
  panic!("input did not contain two ids differing by only one char");
}

fn find_similar_ids(input: &str) -> Option<(String, String, usize)> {
  let mut coll: Vec<String> = vec![];
  for line in input.lines() {
    for comp in coll.iter() {
      if let Some(index) = find_single_difference_id(line, comp) {
        return Some((line.to_owned(), comp.to_owned(), index));
      }
    }
    coll.push(line.to_owned());
  }
  None
}

fn find_single_difference_id(s1: &str, s2: &str) -> Option<usize> {
  let mut differing: Option<usize> = None;
  for (i, (c1, c2)) in s1.chars().zip(s2.chars()).enumerate() {
    if c1 != c2 {
      if differing.is_some() {
        // more than one difference -> wrong pair
        return None;
      }
      differing = Some(i);
    }
  }
  return differing;
}
