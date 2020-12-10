use std::collections::VecDeque;

use crate::util;

pub fn problem_9_2() -> String {
    let file = util::read("input/problem_9_input.txt");
    let lines = file.lines().map(|i| i.parse().unwrap());

    let mut window: VecDeque<usize> = lines.clone().take(25).collect();

    let res = lines.skip(25).find(|curr| {
        let is_valid = (|| {
            for (iidx, i) in window.iter().enumerate() {
                for (jidx, j) in window.iter().enumerate() {
                    if i != j && iidx != jidx && i + j == *curr {
                        return true;
                    }
                }
            }
            return false;
        })();

        window.pop_front();
        window.push_back(*curr);
        !is_valid
    });

    res.unwrap().to_string()
}
