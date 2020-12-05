use crate::util;

pub fn problem_5_1() -> String {
    util::read("input/problem_5_input.txt")
        .lines()
        .map(|i| {
            i.chars().fold(0, |prev, curr| {
                prev * 2
                    + match curr {
                        'B' | 'R' => 1,
                        'F' | 'L' | _ => 0,
                    }
            })
        })
        .max()
        .unwrap()
        .to_string()
}
