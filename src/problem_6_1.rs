use std::iter;

use crate::util;

pub fn problem_6_1() -> String {
    util::read("input/problem_6_input.txt")
        .lines()
        .flat_map(|i| -> Box<dyn Iterator<Item = u8>> {
            let trimmed = i.trim();
            match trimmed.is_empty() {
                true => Box::new(iter::once(' ' as u8)),
                false => Box::new(trimmed.bytes()),
            }
        })
        .fold(vec![[false; u8::MAX as usize]], |mut prev, curr| {
            match curr == ' ' as u8 {
                true => prev.push([false; u8::MAX as usize]),
                false => prev.last_mut().unwrap()[curr as usize] = true,
            };
            prev
        })
        .iter()
        .map(|i| i.iter().filter(|j| **j).count())
        .sum::<usize>()
        .to_string()
}
