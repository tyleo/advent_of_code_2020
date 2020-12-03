use crate::util;

pub fn problem_1() -> String {
    let i32s = util::read("input/problem_1_input.txt")
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

    return (a * b).to_string();
}
