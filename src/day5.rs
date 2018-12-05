#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
  input.chars().fold(Vec::with_capacity(input.len()), |mut vec: Vec<char>, c| {
    if !c.is_alphabetic() {
      return vec
    }
    match vec.last() {
      Some(&c2) if c2 == swap_case(c) => {
        vec.pop();
      }
      _ => vec.push(c)
    };
    vec
  }).len()
}

fn swap_case(chr: char) -> char {
  (chr as u8 ^ 0b100000) as char
}
