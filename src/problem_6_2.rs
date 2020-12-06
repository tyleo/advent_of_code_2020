use crate::util;

pub fn problem_6_2() -> String {
    util::read("input/problem_6_input.txt")
        .lines()
        .fold(vec![([0; u8::MAX as usize], 0)], |mut prev, curr| {
            let trimmed = curr.trim();
            match trimmed.is_empty() {
                true => prev.push(([0; u8::MAX as usize], 0)),
                false => {
                    let (questions, people_count) = prev.last_mut().unwrap();
                    for char in trimmed.bytes() {
                        questions[char as usize] += 1;
                    }
                    *people_count += 1;
                }
            };
            prev
        })
        .iter()
        .map(|(questions, people_count)| questions.iter().filter(|i| *i == people_count).count())
        .sum::<usize>()
        .to_string()
}
