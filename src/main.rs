extern crate regex;

mod day_00;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1
    {
        match &*args[1] {
            "day_zero" => day_00::day_zero(&args),
            "day_one" => day_01::day_one(&args),
            "day_two" => day_02::day_two(&args),
            "day_three" => day_03::day_three(&args),
            "day_four" => day_04::day_four(&args),
            "day_five" => day_05::day_five(&args),
            _ => println!("Not a valid day\n")
        }
    }
    else {
        println!("Need at least one argument");
    }   
}
