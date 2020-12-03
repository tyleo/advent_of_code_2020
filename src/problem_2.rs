use crate::util;

pub fn problem_2() -> String {
    let i32s = util::read("input/problem_2_input.txt")
        .lines()
        .map(|f| f.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let (a, b, c) = i32s
        .iter()
        .fold(None, |prev_a_b_c, a| {
            prev_a_b_c.or_else(|| {
                i32s.iter().fold(None, |prev_a_b_c, b| {
                    prev_a_b_c.or_else(|| {
                        i32s.iter()
                            .filter(|c| a + b + *c == 2020)
                            .nth(0)
                            .map(|c| (a, b, c))
                    })
                })
            })
        })
        .unwrap();

    return (a * b * c).to_string();
}
