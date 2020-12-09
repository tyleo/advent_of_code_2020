use crate::util;

pub fn problem_8_1() -> String {
    let file = util::read("input/problem_8_input.txt");
    let mut lines = file
        .lines()
        .map(|i| (false, &i[..3], i[4..].parse::<i32>().unwrap()))
        .into_iter()
        .collect::<Vec<_>>();

    let mut acc = 0;
    let mut ic = 0;
    while !lines[ic].0 {
        lines[ic].0 = true;
        match lines[ic].1 {
            "nop" => ic += 1,
            "acc" => {
                acc += lines[ic].2;
                ic += 1;
            }
            "jmp" => ic = (ic as i32 + lines[ic].2) as usize,
            _ => {}
        }
    }

    acc.to_string()
}
