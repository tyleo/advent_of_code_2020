use crate::util;

pub fn problem_5_2() -> String {
    let mut nums = util::read("input/problem_5_input.txt")
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
        .collect::<Vec<_>>();

    nums.sort();

    nums.into_iter()
        .fold((None, None), |(res, prev), curr| match res {
            None => match prev {
                None => (None, Some(curr)),
                Some(prev) => {
                    if prev + 1 == curr {
                        (None, Some(curr))
                    } else {
                        (Some(prev + 1), None)
                    }
                }
            },
            i => (i, prev),
        })
        .0
        .unwrap()
        .to_string()
}
