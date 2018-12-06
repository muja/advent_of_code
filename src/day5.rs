#[aoc(day5, part1)]
fn part1(input: &str) -> usize {
    length_after_dissolve(input.chars(), input.len())
}

#[aoc(day5, part2)]
fn part2<'a>(input: &str) -> usize {
    let mut iters = Vec::new();
    for i in 0..26 {
        iters.push(
            input
                .chars()
                .filter(move |c| c.to_ascii_lowercase() as u8 - b'a' != i),
        );
    }
    iters
        .into_iter()
        .map(|it| length_after_dissolve(it, input.len()))
        .min()
        .unwrap()
}

fn length_after_dissolve(it: impl std::iter::Iterator<Item = char>, cap: usize) -> usize {
    it.fold(Vec::with_capacity(cap), |mut vec: Vec<char>, c| {
        if !c.is_alphabetic() {
            return vec;
        }
        match vec.last() {
            Some(&c2) if c2 == swap_case(c) => {
                vec.pop();
            }
            _ => vec.push(c),
        };
        vec
    })
    .len()
}

fn swap_case(chr: char) -> char {
    (chr as u8 ^ 0b100000) as char
}
