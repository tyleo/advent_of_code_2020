use std::collections::VecDeque;

use crate::util;

pub fn problem_9_2() -> String {
    let file = util::read("input/problem_9_input.txt");
    let lines: Vec<_> = file.lines().map(|i| i.parse().unwrap()).collect();

    let mut window: VecDeque<usize> = lines.iter().take(25).copied().collect();

    for curr in lines.iter().skip(25).copied() {
        let is_valid = (|| {
            for (iidx, i) in window.iter().enumerate() {
                for (jidx, j) in window.iter().enumerate() {
                    if i != j && iidx != jidx && i + j == curr {
                        return true;
                    }
                }
            }
            return false;
        })();

        if !is_valid {
            for iidx in 0..lines.len() {
                for jidx in (iidx + 1)..lines.len() {
                    let sum: usize = lines.iter().skip(iidx).take(jidx - iidx + 1).sum();
                    if sum == curr {
                        let range = lines.iter().skip(iidx).take(jidx - iidx + 1);
                        let min = range.clone().min().unwrap();
                        let max = range.max().unwrap();
                        return (min + max).to_string();
                    } else if sum > curr {
                        break;
                    }
                }
            }
        }

        window.pop_front();
        window.push_back(curr);
    }

    "error".to_string()
}
