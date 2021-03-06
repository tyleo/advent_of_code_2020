use crate::util;

pub fn problem_5_2() -> String {
    let mut nums = util::read("input/problem_5_input.txt")
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
        .collect::<Vec<_>>();

    nums.sort();

    nums.into_iter()
        .fold((None, None), |(res, prev), curr| match (res, prev) {
            (None, None) => (None, Some(curr)),
            (None, Some(prev)) if prev + 1 == curr => (None, Some(curr)),
            (None, Some(prev)) if prev + 1 != curr => (Some(prev + 1), None),
            i => i,
        })
        .0
        .unwrap()
        .to_string()
}
