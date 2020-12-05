use crate::util;

pub fn problem_5_1() -> String {
    util::read("input/problem_5_input.txt")
        .lines()
        .map(|i| {
            i.chars().fold(0, |prev, curr| {
                prev * 2
                    + match curr {
                        'F' => 0,
                        'B' => 1,
                        'L' => 0,
                        'R' => 1,
                        _ => 0,
                    }
            })
        })
        .max()
        .unwrap()
        .to_string()
}
