use crate::util;

pub fn problem_3_1() -> String {
    util::read("input/problem_3_input.txt")
        .lines()
        .enumerate()
        .map(|(y, row)| row.chars().cycle().nth(y * 3).unwrap() == '#')
        .filter(|i| *i)
        .count()
        .to_string()
}
