use crate::util;

pub fn problem_7_1() -> String {
    // util::read("input/problem_7_input.txt")
    //     .lines()
    //     .fold(vec![([0; u8::MAX as usize], 0)], |mut prev, curr| {
    //         let trimmed = curr.trim();
    //         if trimmed.is_empty() {
    //             prev.push(([0; u8::MAX as usize], 0))
    //         } else {
    //             let (questions, people_count) = prev.last_mut().unwrap();
    //             for char in trimmed.bytes() {
    //                 questions[char as usize] += 1;
    //             }
    //             *people_count += 1;
    //         };
    //         prev
    //     })
    //     .iter()
    //     .map(|(questions, people_count)| questions.iter().filter(|i| *i == people_count).count())
    //     .sum::<usize>()
    //     .to_string()
    "".to_string()
}
