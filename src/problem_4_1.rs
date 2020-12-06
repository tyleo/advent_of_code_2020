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
        .fold(vec![File::default()], |mut prev, curr| {
            match curr.is_empty() {
                true => prev.push(File::default()),
                false => match curr.chars().take(3).collect::<String>().as_str() {
                    "byr" => prev.last_mut().unwrap().byr = true,
                    "iyr" => prev.last_mut().unwrap().iyr = true,
                    "eyr" => prev.last_mut().unwrap().eyr = true,
                    "hgt" => prev.last_mut().unwrap().hgt = true,
                    "hcl" => prev.last_mut().unwrap().hcl = true,
                    "ecl" => prev.last_mut().unwrap().ecl = true,
                    "pid" => prev.last_mut().unwrap().pid = true,
                    "cid" => prev.last_mut().unwrap().cid = true,
                    _ => {}
                },
            };
            prev
        })
        .into_iter()
        .filter(|i| i.byr && i.iyr && i.eyr && i.hgt && i.hcl && i.ecl && i.pid)
        .count()
        .to_string()
}
