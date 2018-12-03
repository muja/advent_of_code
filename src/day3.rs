#[aoc(day3, part1)]
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
  let mut counter = 0;
  for x in fabric.iter() {
    counter += x.iter().filter(|&&e| e > 1).count();
  }
  counter
}
