use crate::util;

pub fn problem_10_2() -> String {
    let mut jolt_ratings: Vec<i32> = util::read("input/problem_10_input.txt")
        .lines()
        .map(|i| i.parse().unwrap())
        .collect();
    jolt_ratings.sort();
    jolt_ratings
        .iter()
        .copied()
        .fold(
            (Some(0), 1u128, None, 0u128, None, 0u128),
            |(last, last_paths, second_last, second_last_paths, third_last, third_last_paths),
             curr| {
                let mut curr_paths = 0;

                if let Some(last) = last {
                    if curr - last <= 3 {
                        curr_paths += last_paths;
                    }
                }

                if let Some(second_last) = second_last {
                    if curr - second_last <= 3 {
                        curr_paths += second_last_paths;
                    }
                }

                if let Some(third_last) = third_last {
                    if curr - third_last <= 3 {
                        curr_paths += third_last_paths;
                    }
                }

                (
                    Some(curr),
                    curr_paths,
                    last,
                    last_paths,
                    second_last,
                    second_last_paths,
                )
            },
        )
        .1
        .to_string()
}
