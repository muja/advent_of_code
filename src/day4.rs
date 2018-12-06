use std::collections::HashMap;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> i32 {
    let grid = guard_asleep_table(input);
    let (id, minutes) = grid
        .into_iter()
        .max_by_key(|(_, v1)| v1.into_iter().map(|(id, m)| m - id).sum::<i32>())
        .unwrap();
    let mut frequency = [0i32; 60];
    for (start, end) in minutes {
        for i in start..end {
            frequency[i as usize] += 1;
        }
    }
    frequency
        .iter()
        .enumerate()
        .max_by_key(|&(_, e)| e)
        .expect("max minute")
        .0 as i32
        * id
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> i32 {
    let grid = guard_asleep_table(input);
    let (id, minute, _) = grid
        .iter()
        .map(|(k, v)| {
            let mut frequency = [0i32; 60];
            for &(start, end) in v {
                for i in start..end {
                    frequency[i as usize] += 1;
                }
            }
            let (minute, &times) = frequency
                .iter()
                .enumerate()
                .max_by_key(|&(_, e)| e)
                .expect("max minute");
            (k, minute, times)
        })
        .max_by_key(|&(_, _, times)| times)
        .expect("max times");
    id * minute as i32
}

fn guard_asleep_table(input: &str) -> HashMap<i32, Vec<(i32, i32)>> {
    let mut vec = input.lines().collect::<Vec<_>>();
    vec.sort();
    let mut guard_grid: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    vec.into_iter()
        .scan((-1, -1), |state, line| {
            match &line[19..20] {
                "G" => {
                    // new Guard
                    let id = line
                        .split_at(26)
                        .1
                        .chars()
                        .take_while(|c| c.is_numeric())
                        .collect::<String>()
                        .parse::<i32>()
                        .expect("guard id parse");
                    state.0 = id;
                }
                "f" => {
                    // falls asleep
                    assert!(state.0 >= 0); // falls asleep without guard => panic
                    let sleeps_at_minute: i32 = line[15..17].parse().expect("sleep minute parse");
                    state.1 = sleeps_at_minute;
                }
                "w" => {
                    // wakes up
                    assert!(state.0 >= 0); // wakes up without guarding/sleeping first => panic
                    assert!(state.1 >= 0); // wakes up without guarding/sleeping first => panic
                    let wakes_at_minute: i32 = line[15..17].parse().expect("wake minute parse");
                    guard_grid
                        .entry(state.0)
                        .or_default()
                        .push((state.1, wakes_at_minute));
                }
                _ => panic!("unexpected character"),
            };
            Some(0)
        })
        .for_each(|e| assert_eq!(e, 0));
    guard_grid
}
