use crate::util;

pub fn problem_10_1() -> String {
    let mut jolt_ratings: Vec<i32> = util::read("input/problem_10_input.txt")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();
    jolt_ratings.sort();
    let (_, ones, _, threes) = (0..1).chain(jolt_ratings.iter().copied()).fold(
        (None, 0, 0, 1),
        |(last, mut ones, mut twos, mut threes), curr| {
            if let Some(last) = last {
                match curr - last {
                    1 => ones += 1,
                    2 => twos += 1,
                    3 => threes += 1,
                    _ => {}
                }
            }
            (Some(curr), ones, twos, threes)
        },
    );
    (ones * threes).to_string()
}
