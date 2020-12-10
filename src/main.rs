extern crate regex;

mod day_00;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
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
            "day_six" => day_06::day_six(&args),
            "day_seven" => day_07::day_seven(&args),
            "day_eight" => day_08::day_eight(&args),
            "day_nine" => day_09::day_nine(&args),
            _ => println!("Not a valid day\n")
        }
    }
    else {
        println!("Need at least one argument");
    }   
}
