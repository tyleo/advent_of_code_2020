#[macro_use]
extern crate lazy_static;
extern crate regex;

mod problem_1_1;
mod problem_1_2;
mod problem_2_1;
mod problem_2_2;
mod util;

fn main() {
    println!("problem_1_1: {}", problem_1_1::problem_1_1());
    println!("problem_1_2: {}", problem_1_2::problem_1_2());
    println!("problem_2_1: {}", problem_2_1::problem_2_1());
    println!("problem_2_2: {}", problem_2_2::problem_2_2());
}
