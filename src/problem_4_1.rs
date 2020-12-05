use crate::util;

#[derive(Default)]
struct File {
    pub byr: bool,
    pub iyr: bool,
    pub eyr: bool,
    pub hgt: bool,
    pub hcl: bool,
    pub ecl: bool,
    pub pid: bool,
    pub cid: bool,
}

pub fn problem_4_1() -> String {
    util::read("input/problem_4_input.txt")
        .lines()
        .flat_map(|i| i.split(" "))
        .fold(
            (Vec::new(), File::default()),
            |(mut prev_vec, mut prev_file), curr| {
                if curr.is_empty() {
                    prev_vec.push(prev_file);
                    (prev_vec, File::default())
                } else {
                    match curr.chars().take(3).collect::<String>().as_str() {
                        "byr" => prev_file.byr = true,
                        "iyr" => prev_file.iyr = true,
                        "eyr" => prev_file.eyr = true,
                        "hgt" => prev_file.hgt = true,
                        "hcl" => prev_file.hcl = true,
                        "ecl" => prev_file.ecl = true,
                        "pid" => prev_file.pid = true,
                        "cid" => prev_file.cid = true,
                        _ => {}
                    };
                    (prev_vec, prev_file)
                }
            },
        )
        .0
        .into_iter()
        .filter(|i| i.byr && i.iyr && i.eyr && i.hgt && i.hcl && i.ecl && i.pid)
        .count()
        .to_string()
}
