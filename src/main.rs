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
mod problem_5_2;
mod problem_6_1;
mod problem_6_2;
mod problem_7_1;
mod problem_7_2;
mod problem_8_1;
mod problem_8_2;
mod problem_9_1;
mod problem_9_2;
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
    println!("problem_5_2: {}", problem_5_2::problem_5_2());
    println!("problem_6_1: {}", problem_6_1::problem_6_1());
    println!("problem_6_2: {}", problem_6_2::problem_6_2());
    println!("problem_7_1: {}", problem_7_1::problem_7_1());
    println!("problem_7_2: {}", problem_7_2::problem_7_2());
    println!("problem_8_1: {}", problem_8_1::problem_8_1());
    println!("problem_8_2: {}", problem_8_2::problem_8_2());
    println!("problem_9_1: {}", problem_9_1::problem_9_1());
    println!("problem_9_2: {}", problem_9_2::problem_9_2());
}
