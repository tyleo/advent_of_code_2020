#[macro_use]
extern crate lazy_static;
extern crate regex;

mod problem_1_1;
mod problem_1_2;
mod problem_2_1;
mod problem_2_2;
mod problem_3_1;
mod problem_3_2;
mod problem_4_1;
mod problem_4_2;
mod problem_5_1;
mod util;

fn main() {
    println!("problem_1_1: {}", problem_1_1::problem_1_1());
    println!("problem_1_2: {}", problem_1_2::problem_1_2());
    println!("problem_2_1: {}", problem_2_1::problem_2_1());
    println!("problem_2_2: {}", problem_2_2::problem_2_2());
    println!("problem_3_1: {}", problem_3_1::problem_3_1());
    println!("problem_3_2: {}", problem_3_2::problem_3_2());
    println!("problem_4_1: {}", problem_4_1::problem_4_1());
    println!("problem_4_2: {}", problem_4_2::problem_4_2());
    println!("problem_5_1: {}", problem_5_1::problem_5_1());
}
