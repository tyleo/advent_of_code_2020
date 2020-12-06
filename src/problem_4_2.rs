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

pub fn problem_4_2() -> String {
    util::read("input/problem_4_input.txt")
        .lines()
        .flat_map(|i| i.split(" "))
        .fold(vec![File::default()], |mut prev, curr| {
            match curr.is_empty() {
                true => prev.push(File::default()),
                false => match curr.chars().take(3).collect::<String>().as_str() {
                    "byr" => {
                        let byr = curr.chars().skip(4).collect::<String>().parse().unwrap();
                        prev.last_mut().unwrap().byr = 1920 <= byr && byr <= 2020
                    }
                    "iyr" => {
                        let iyr = curr.chars().skip(4).collect::<String>().parse().unwrap();
                        prev.last_mut().unwrap().iyr = 2010 <= iyr && iyr <= 2020
                    }
                    "eyr" => {
                        let eyr = curr.chars().skip(4).collect::<String>().parse().unwrap();
                        prev.last_mut().unwrap().eyr = 2020 <= eyr && eyr <= 2030
                    }
                    "hgt" => {
                        let hgt_num_str = curr
                            .chars()
                            .skip(4)
                            .filter(|i| i.is_numeric())
                            .collect::<String>();
                        let height_num = hgt_num_str.parse().unwrap();
                        prev.last_mut().unwrap().hgt = match curr
                            .chars()
                            .skip(4 + hgt_num_str.len())
                            .collect::<String>()
                            .as_str()
                        {
                            "cm" => 150 <= height_num && height_num <= 193,
                            "in" => 59 <= height_num && height_num <= 76,
                            _ => false,
                        }
                    }
                    "hcl" => {
                        prev.last_mut().unwrap().hcl =
                            if curr.chars().skip(4).nth(0).unwrap() != '#' {
                                false
                            } else {
                                curr.chars()
                                    .skip(4 + 1)
                                    .filter(|i| i.is_numeric() || ('a' <= *i && *i <= 'f'))
                                    .count()
                                    == 6
                            }
                    }
                    "ecl" => {
                        prev.last_mut().unwrap().ecl =
                            match curr.chars().skip(4).collect::<String>().as_str() {
                                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                                _ => false,
                            }
                    }
                    "pid" => {
                        let passport_id = curr.chars().skip(4).collect::<String>();
                        prev.last_mut().unwrap().pid =
                            passport_id.chars().filter(|i| i.is_numeric()).count() == 9;
                    }
                    "cid" => prev.last_mut().unwrap().cid = true,
                    _ => {}
                },
            }
            prev
        })
        .iter()
        .filter(|i| i.byr && i.iyr && i.eyr && i.hgt && i.hcl && i.ecl && i.pid)
        .count()
        .to_string()
}
