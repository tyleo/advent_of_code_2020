use crate::util;

#[derive(Default)]
struct File {
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub eyr: Option<String>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl File {
    pub fn is_valid(&self) -> bool {
        self.byr
            .as_deref()
            .and(self.iyr.as_deref())
            .and(self.eyr.as_deref())
            .and(self.hgt.as_deref())
            .and(self.hcl.as_deref())
            .and(self.ecl.as_deref())
            .and(self.pid.as_deref())
            .is_some()
    }
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
                        "byr" => prev_file.byr = Some(curr.chars().skip(4).collect::<String>()),
                        "iyr" => prev_file.iyr = Some(curr.chars().skip(4).collect::<String>()),
                        "eyr" => prev_file.eyr = Some(curr.chars().skip(4).collect::<String>()),
                        "hgt" => prev_file.hgt = Some(curr.chars().skip(4).collect::<String>()),
                        "hcl" => prev_file.hcl = Some(curr.chars().skip(4).collect::<String>()),
                        "ecl" => prev_file.ecl = Some(curr.chars().skip(4).collect::<String>()),
                        "pid" => prev_file.pid = Some(curr.chars().skip(4).collect::<String>()),
                        "cid" => prev_file.cid = Some(curr.chars().skip(4).collect::<String>()),
                        _ => {}
                    };
                    (prev_vec, prev_file)
                }
            },
        )
        .0
        .into_iter()
        .filter(File::is_valid)
        .count()
        .to_string()
}
