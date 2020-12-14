use std::fs;
use std::collections::HashMap;

const MAXIMUM: u64 = u64::max_value() >> (64 - 36);

fn get_or_mask(mask: &String) -> u64 {
    return u64::from_str_radix(mask.replace("X","0").as_str(),2).unwrap();
}

fn get_and_mask(mask: &String) -> u64 {
    return u64::from_str_radix(mask.replace("X","1").as_str(),2).unwrap();
}

fn get_floating_mask(mask: &String) -> u64 {
    return get_or_mask(mask) | !get_and_mask(mask);
}

fn apply_mask(mask: &String, value:u64) -> u64 {
    let and_mask = get_and_mask(mask);
    let or_mask = get_or_mask(mask);

    return (value & and_mask) | or_mask;
}

pub fn day_fourteen(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let mut split = contents.split("\n");

        let mut current_mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".to_string();
        let mut mem = HashMap::new();

        for row in split {
            let row_parts : Vec<&str> = row.split(" = ").collect();

            if row_parts[0].starts_with("mask") {
                current_mask = row_parts[1].to_string();
            } else {
                let mem_key = &row_parts[0].to_string();
                let current_memory_pointer = mem_key[4..mem_key.len()-1].parse::<u64>().unwrap();
                let current_value = row_parts[1].to_string().parse::<u64>().unwrap();
                let output_value = apply_mask(&current_mask,current_value);
                mem.insert(current_memory_pointer, output_value);
            }
        }

        let mut current_total = 0;

        for current_value in mem.values() {
            current_total += current_value;
        }

        println!("part 1: {}", current_total);

        mem = HashMap::new();
        split = contents.split("\n");

        for row in split {
            let row_parts : Vec<&str> = row.split(" = ").collect();

            if row_parts[0].starts_with("mask") {
                current_mask = row_parts[1].to_string();
            } else {
                let mem_key = &row_parts[0].to_string();
                let current_memory_pointer = mem_key[4..mem_key.len()-1].parse::<u64>().unwrap();
                let current_value = row_parts[1].to_string().parse::<u64>().unwrap();
                let my_pointer = (current_memory_pointer | get_or_mask(&current_mask)) & get_floating_mask(&current_mask);
                let mut floating_mask = get_floating_mask(&current_mask);

                loop {
                    let inverted_floating_mask = (!floating_mask) & MAXIMUM;
                    let floating_pointer = my_pointer | inverted_floating_mask;
                    mem.insert(floating_pointer, current_value);

                    if floating_mask & MAXIMUM == MAXIMUM {
                        break;
                    }

                    floating_mask += 1;
                    floating_mask |= get_floating_mask(&current_mask);
                }

                // let output_value = apply_mask(&current_mask,current_value);
                // mem.insert(current_memory_pointer, output_value);
            }
        }

        let mut current_total = 0;

        for current_value in mem.values() {
            current_total += current_value;
        }

        println!("part 2: {}", current_total);
    }
    else
    {
        println!("day_fourteen needs 2 args\n");
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mask() {
        assert_eq!(apply_mask(&"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),11),73);
        assert_eq!(apply_mask(&"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),101),101);
        assert_eq!(apply_mask(&"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),0),64);
    }
}