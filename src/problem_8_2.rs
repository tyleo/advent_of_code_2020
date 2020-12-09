use crate::util;

pub fn problem_8_2() -> String {
    let file = util::read("input/problem_8_input.txt");
    let mut lines = file
        .lines()
        .map(|i| (false, &i[..3], i[4..].parse::<i32>().unwrap(), false))
        .into_iter()
        .collect::<Vec<_>>();

    loop {
        let mut did_swap = false;
        let mut ic = 0;
        let mut acc = 0;

        while ic < lines.len() && !lines[ic].0 {
            lines[ic].0 = true;
            match lines[ic].1 {
                "nop" => {
                    if !did_swap && !lines[ic].3 {
                        did_swap = true;
                        lines[ic].3 = true;
                        ic = (ic as i32 + lines[ic].2) as usize;
                    } else {
                        ic += 1
                    }
                }
                "acc" => {
                    acc += lines[ic].2;
                    ic += 1;
                }
                "jmp" => {
                    if !did_swap && !lines[ic].3 {
                        did_swap = true;
                        lines[ic].3 = true;
                        ic += 1;
                    } else {
                        ic = (ic as i32 + lines[ic].2) as usize
                    }
                }
                _ => {}
            }
        }

        for line in &mut lines {
            *line = (false, line.1, line.2, line.3)
        }

        if ic >= lines.len() {
            return acc.to_string();
        }
    }
}
