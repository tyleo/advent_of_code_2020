#[macro_use]
extern crate lazy_static;
extern crate regex;

mod problem_1;
mod problem_2;
mod problem_3;
mod util;

fn main() {
    println!("problem_1: {}", problem_1::problem_1());
    println!("problem_2: {}", problem_2::problem_2());
    println!("problem_3: {}", problem_3::problem_3());
}
