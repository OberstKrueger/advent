mod answer;
mod solutions2015;
mod solutions2024;

use answer::*;
use solutions2015::*;
use solutions2024::*;

use std::fs::read_to_string;

fn main() {
    println!("2015.01 => {}", solution_2015_01(&read_file("inputs/2015_01.txt")));
    println!("2015.02 => {}", solution_2015_02(&read_file("inputs/2015_02.txt")));

    print!("\n");
    println!("2024.01 => {}", solution_2024_01(&read_file("inputs/2024_01.txt")));
    println!("2024.02 => {}", solution_2024_02(&read_file("inputs/2024_02.txt")));
}

fn read_file(input: &str) -> String {
    let text: String;

    text = read_to_string(input).expect("Could not read input");

    text
}
