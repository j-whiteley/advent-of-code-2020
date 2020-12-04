use std::fs;
use std::collections::HashMap;
use regex::Regex;

const VALID_KEYS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_EYE_COLOURS: &'static [&'static str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn validate_passport(passport: &HashMap<&str,&str>) -> bool {

    let mut is_valid = true;

    for valid_value in VALID_KEYS.iter() {
        is_valid = is_valid && passport.contains_key(valid_value);
    }

    return is_valid;
}

fn validate_year(year: &str, min_year: usize, max_year: usize) -> bool {
    let year_as_num = year.parse::<usize>().unwrap();

    return year_as_num >= min_year && year_as_num <= max_year;
}

fn validate_height(height: &str) -> bool {
    let mut is_valid = false;
    if height.ends_with("cm") {
        let height_as_num = height.split("cm").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();

        is_valid = height_as_num >= 150 && height_as_num <= 193;

    } else if height.ends_with("in") {
        let height_as_num = height.split("in").collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
        
        is_valid = height_as_num >= 59 && height_as_num <= 76;
    }

    return is_valid;
}

fn further_validate_passport(passport: &HashMap<&str,&str>) -> bool {

    let mut is_valid = true;

    let pid_regex = Regex::new(r"^\d{9}$").unwrap();
    let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    for valid_value in VALID_KEYS.iter() {
        if passport.contains_key(valid_value) {
            let to_validate = passport.get(valid_value).unwrap();
            let mut this_valid = false;

            match valid_value {
                &"byr" => this_valid = validate_year(to_validate, 1920,2002), 
                &"iyr" => this_valid = validate_year(to_validate, 2010,2020), 
                &"eyr" => this_valid = validate_year(to_validate, 2020,2030), 
                &"hgt" => this_valid = validate_height(to_validate), 
                &"hcl" => this_valid = hcl_regex.is_match(to_validate), 
                &"ecl" => this_valid = VALID_EYE_COLOURS.contains(to_validate), 
                &"pid" => this_valid = pid_regex.is_match(to_validate),
                _ => {}
            }
            
            is_valid = is_valid && this_valid;
        } else {
            is_valid = false;
        }
    }

    return is_valid;
}

pub fn day_four(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        
        let split = contents.split("\n");

        let mut passports = Vec::new();
        let mut current_passport = HashMap::new();

        for line in split {
            if line.len() < 3 {
                //Complete Pass port
                passports.push(current_passport);
                current_passport = HashMap::new();

            } else {
                let pairs = line.split_whitespace();

                for pair in pairs {
                    let values = pair.split(":").collect::<Vec<&str>>();

                    current_passport.insert(values[0],values[1]);
                }
            }
        }

        //Add final passport
        passports.push(current_passport);

        println!("Found {} passports\n", passports.len());

        let mut valid_count = 0;
        let mut further_valid_count = 0;

        for passport in passports.iter() {
            if validate_passport(passport) {
                valid_count += 1;

                if further_validate_passport(passport) {
                    further_valid_count += 1;
                }
            }
        }

        println!("Got {} valid passports\n", valid_count);
        println!("Got {} super valid passports\n", further_valid_count);
    }
    else
    {
        println!("day_four needs 2 args\n");
    }
}