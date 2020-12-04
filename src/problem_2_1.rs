use regex;
use std::fmt;
use std::str::FromStr;

use crate::util;

struct PasswordPattern {
    lower_char_limit: usize,
    upper_char_limit: usize,
    char: char,
    password: String,
}

lazy_static! {
    static ref PASSWORD_PATTERN_REGEX: regex::Regex =
        regex::Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
}

impl FromStr for PasswordPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let password_captures = PASSWORD_PATTERN_REGEX
            .captures_iter(s)
            .nth(0)
            .ok_or_else(|| "Regex failed to match")?;

        fn parse<T: FromStr>(
            captures: &regex::Captures,
            idx: usize,
            name: &str,
        ) -> Result<T, String>
        where
            T::Err: fmt::Display,
        {
            captures
                .get(idx)
                .ok_or_else(|| format!("{} not found", name))?
                .as_str()
                .parse::<T>()
                .map_err(|err| format!("{} parse failed: {}", name, err))
        };

        let (lower_char_limit, upper_char_limit, char, password) = (
            parse(&password_captures, 1, "lower_char_limit")?,
            parse(&password_captures, 2, "upper_char_limit")?,
            parse(&password_captures, 3, "char")?,
            parse(&password_captures, 4, "password")?,
        );
        Ok(Self {
            lower_char_limit,
            upper_char_limit,
            char,
            password,
        })
    }
}

pub fn problem_2_1() -> String {
    let passwords = util::read("input/problem_2_input.txt")
        .lines()
        .map(|f| f.parse::<PasswordPattern>().unwrap())
        .collect::<Vec<_>>();

    return passwords
        .iter()
        .filter(|password_pattern| {
            let matching_chars = password_pattern
                .password
                .chars()
                .filter(|password_char| *password_char == password_pattern.char)
                .count();
            matching_chars <= password_pattern.upper_char_limit
                && matching_chars >= password_pattern.lower_char_limit
        })
        .count()
        .to_string();
}
