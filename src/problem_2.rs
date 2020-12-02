use std::fs;
use std::io::prelude::*;

pub fn problem_2() -> String {
    let mut file = fs::File::open("input/problem_2_input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let i32s = contents
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
        //     None => i32s.iter().fold(None, |prev_a_b_c, curr|)
        //         .iter()
        //         .filter(|inner| curr + *inner == 2020)
        //         .nth(0)
        //         .map(|inner| (curr, inner)),
        //     prev => prev,
        // })
        .unwrap();
    return format!("{}", a * b * c);
}
