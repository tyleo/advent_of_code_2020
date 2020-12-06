use crate::util;

pub fn problem_6_1() -> String {
    util::read("input/problem_6_input.txt")
        .lines()
        .fold(vec![[false; u8::MAX as usize]], |mut prev, curr| {
            let trimmed = curr.trim();
            match trimmed.is_empty() {
                true => prev.push([false; u8::MAX as usize]),
                false => {
                    for char in trimmed.bytes() {
                        prev.last_mut().unwrap()[char as usize] = true
                    }
                }
            };
            prev
        })
        .iter()
        .map(|i| i.iter().filter(|j| **j).count())
        .sum::<usize>()
        .to_string()
}
