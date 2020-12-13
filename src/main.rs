#[macro_use]
extern crate lazy_static;
extern crate regex;

mod problem_10_1;
mod problem_10_2;
mod problem_11_1;
mod problem_11_2;
mod problem_12_1;
mod problem_12_2;
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

use std::io;

fn main() {
    println!("Enter a problem like x.y:");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    match buf.trim() {
        "1.1" => println!("problem_1_1: {}", problem_1_1::problem_1_1()),
        "1.2" => println!("problem_1_2: {}", problem_1_2::problem_1_2()),
        "2.1" => println!("problem_2_1: {}", problem_2_1::problem_2_1()),
        "2.2" => println!("problem_2_2: {}", problem_2_2::problem_2_2()),
        "3.1" => println!("problem_3_1: {}", problem_3_1::problem_3_1()),
        "3.2" => println!("problem_3_2: {}", problem_3_2::problem_3_2()),
        "4.1" => println!("problem_4_1: {}", problem_4_1::problem_4_1()),
        "4.2" => println!("problem_4_2: {}", problem_4_2::problem_4_2()),
        "5.1" => println!("problem_5_1: {}", problem_5_1::problem_5_1()),
        "5.2" => println!("problem_5_2: {}", problem_5_2::problem_5_2()),
        "6.1" => println!("problem_6_1: {}", problem_6_1::problem_6_1()),
        "6.2" => println!("problem_6_2: {}", problem_6_2::problem_6_2()),
        "7.1" => println!("problem_7_1: {}", problem_7_1::problem_7_1()),
        "7.2" => println!("problem_7_2: {}", problem_7_2::problem_7_2()),
        "8.1" => println!("problem_8_1: {}", problem_8_1::problem_8_1()),
        "8.2" => println!("problem_8_2: {}", problem_8_2::problem_8_2()),
        "9.1" => println!("problem_9_1: {}", problem_9_1::problem_9_1()),
        "9.2" => println!("problem_9_2: {}", problem_9_2::problem_9_2()),
        "10.1" => println!("problem_10_1: {}", problem_10_1::problem_10_1()),
        "10.2" => println!("problem_10_2: {}", problem_10_2::problem_10_2()),
        "11.1" => println!("problem_11_1: {}", problem_11_1::problem_11_1()),
        "11.2" => println!("problem_11_2: {}", problem_11_2::problem_11_2()),
        "12.1" => println!("problem_12_1: {}", problem_12_1::problem_12_1()),
        "12.2" => println!("problem_12_2: {}", problem_12_2::problem_12_2()),
        _ => println!("Problem could not be found."),
    }
}
