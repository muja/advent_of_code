use std::str::FromStr;
use std::collections::HashMap;

#[aoc(day3, part1, Array)]
pub fn part1(input: &str) -> usize {
  let mut fabric = [[0usize; 1000]; 1000];
  for (x, y, w, h) in input.lines().map(|line| {
    // let [pos, area] = ... unfortunately not possible
    let pos_and_area: Vec<&str> = line.split(" @ ").nth(1).unwrap().split(": ").collect();
    let pos = pos_and_area[0];
    let area = pos_and_area[1];

    // let [x, y] = ... unfortunately not possible
    let x_y: Vec<usize> = pos.split(",").map(|e| e.parse::<usize>().unwrap()).collect();
    let x = x_y[0];
    let y = x_y[1];

    // let [w, h] = ... unfortunately not possible
    let w_h: Vec<usize> = area.split("x").map(|e| e.parse::<usize>().unwrap()).collect();
    let w = w_h[0];
    let h = w_h[1];
    (x, y, w, h)
  }) {
    for i in 0..w {
      for j in 0..h {
        fabric[x+i][y+j] += 1;
      }
    }
  }
  fabric.iter().map(|x| x.iter().filter(|&&e| e > 1).count()).sum()
}

#[aoc(day3, part1, HashMap)]
pub fn part1_map(input: &str) -> usize {
  let mut fabric: HashMap<(usize, usize), usize> = HashMap::new();
  for (x, y, w, h) in input.lines().map(|line| {
    // let [pos, area] = ... unfortunately not possible
    let pos_and_area: Vec<&str> = line.split(" @ ").nth(1).unwrap().split(": ").collect();
    let pos = pos_and_area[0];
    let area = pos_and_area[1];

    // let [x, y] = ... unfortunately not possible
    let x_y: Vec<usize> = pos.split(",").map(|e| e.parse::<usize>().unwrap()).collect();
    let x = x_y[0];
    let y = x_y[1];

    // let [w, h] = ... unfortunately not possible
    let w_h: Vec<usize> = area.split("x").map(|e| e.parse::<usize>().unwrap()).collect();
    let w = w_h[0];
    let h = w_h[1];
    (x, y, w, h)
  }) {
    for i in 0..w {
      for j in 0..h {
        let key = (x+i, y+j);
        if let Some(i) = fabric.insert(key, 1) {
          fabric.insert(key, i + 1);
        }
      }
    }
  }
  fabric.values().filter(|&&e| e > 1).count()
}

struct Claim(isize, usize, usize, usize, usize);

impl FromStr for Claim {
  type Err = &'static str;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    let (id, pos_and_area) = line.split_at(line.find("@").ok_or("err at @")?);
    let id = id.split_at(1).1.trim().parse().map_err(|_| "err at id.split")?;

    let (pos, area) = pos_and_area.split_at(pos_and_area.find(":").ok_or("err at :")?);
    let pos = pos.trim();
    let area = area.trim();

    let (x, y) = pos.split_at(pos.find(",").ok_or("err at (x, y)")?);
    let x = x.split_at(1).1.trim().parse().map_err(|_| "err at x")?;
    let y = y.split_at(1).1.trim().parse().map_err(|_| "err at y")?;

    let (w, h) = area.split_at(area.find("x").ok_or("err at (w, h)")?);
    let w = w.split_at(1).1.trim().parse().map_err(|_| "err at w")?;
    let h = h.split_at(1).1.trim().parse().map_err(|_| "err at h")?;
    Ok(Claim(id, x, y, w, h))
  }
}

#[aoc(day3, part1, Claim)]
pub fn part1_claim(input: &str) -> usize {
  let mut fabric = [[0usize; 1000]; 1000];
  for Claim(_, x, y, w, h) in input.lines().map(|line| line.parse::<Claim>().unwrap()) {
    for i in 0..w {
      for j in 0..h {
        fabric[x+i][y+j] += 1;
      }
    }
  }
  fabric.iter().map(|x| x.iter().filter(|&&e| e > 1).count()).sum()
}

struct RgxClaim(isize, usize, usize, usize, usize);

impl FromStr for RgxClaim {
  type Err = &'static str;

  fn from_str(line: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref RE: regex::Regex = regex::Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    }
    let captures = RE.captures(line).ok_or("capture")?;
    let id: isize = captures.get(1).ok_or("capture id")?.as_str().parse().map_err(|_| "parse id")?;
    let x: usize = captures.get(2).ok_or("capture x")?.as_str().parse().map_err(|_| "parse x")?;
    let y: usize = captures.get(3).ok_or("capture y")?.as_str().parse().map_err(|_| "parse y")?;
    let w: usize = captures.get(4).ok_or("capture w")?.as_str().parse().map_err(|_| "parse w")?;
    let h: usize = captures.get(5).ok_or("capture h")?.as_str().parse().map_err(|_| "parse h")?;
    Ok(RgxClaim(id, x, y, w, h))
  }
}

#[aoc(day3, part1, Regex)]
pub fn part1_regex(input: &str) -> usize {
  let mut fabric = [[0usize; 1000]; 1000];
  for RgxClaim(_, x, y, w, h) in input.lines().map(|line| line.parse::<RgxClaim>().unwrap()) {
    for i in 0..w {
      for j in 0..h {
        fabric[x+i][y+j] += 1;
      }
    }
  }
  fabric.iter().map(|x| x.iter().filter(|&&e| e > 1).count()).sum()
}
