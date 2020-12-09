use crate::util;

pub fn problem_8_2() -> String {
    let file = util::read("input/problem_8_input.txt");
    let mut lines = file
        .lines()
        .map(|i| (0, &i[..3], i[4..].parse::<i32>().unwrap(), false))
        .into_iter()
        .collect::<Vec<_>>();

    let mut run = 0;
    loop {
        run += 1;
        let mut did_swap = false;
        let mut ic = 0;
        let mut acc = 0;

        while ic < lines.len() && lines[ic].0 != run {
            lines[ic].0 = run;
            let mut cmd = lines[ic].1;
            if !did_swap && !lines[ic].3 {
                lines[ic].3 = true;
                if cmd == "nop" {
                    did_swap = true;
                    cmd = "jmp"
                } else if cmd == "jmp" {
                    did_swap = true;
                    cmd = "nop"
                }
            }

            match cmd {
                "nop" => ic += 1,
                "acc" => {
                    acc += lines[ic].2;
                    ic += 1;
                }
                "jmp" => ic = (ic as i32 + lines[ic].2) as usize,
                _ => {}
            }
        }

        if ic >= lines.len() {
            return acc.to_string();
        }
    }
}
