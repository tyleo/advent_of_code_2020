use std::fs;
use std::io::prelude::*;

pub fn problem_1() -> String {
    let mut file = fs::File::open("input/problem_1_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let i32s = contents
        .lines()
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let (a, b) = i32s
        .iter()
        .fold(None, |prev_a_b, a| {
            prev_a_b.or_else(|| {
                i32s.iter()
                    .filter(|b| a + *b == 2020)
                    .nth(0)
                    .map(|b| (a, b))
            })
        })
        .unwrap();

    return format!("{}", a * b);
}
