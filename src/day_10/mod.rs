use std::fs;
use std::collections::HashMap;

pub fn day_ten(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split = contents.split("\n");

        let mut numbers = Vec::new();

        for item in split {
            numbers.push(item.parse::<i32>().unwrap());
        }

        numbers.push(0);

        numbers.sort();

        println!("Max Joltage {}", numbers.last().unwrap() + 3);

        let mut joltage_jumps = HashMap::new();
        let mut previous_number = 0;
        joltage_jumps.insert(&1, 0);
        joltage_jumps.insert(&2, 0);
        joltage_jumps.insert(&3, 0);

        for number in &numbers {
            // let current_number = number.unwrap();
            match number - previous_number {
                1 => joltage_jumps.insert(&1, joltage_jumps.get(&1).unwrap() + 1),
                2 => joltage_jumps.insert(&2, joltage_jumps.get(&2).unwrap() + 1),
                3 => joltage_jumps.insert(&3, joltage_jumps.get(&3).unwrap() + 1),
                _ => None::<i32>
            };

            previous_number = *number;
        }

        let one_jumps =  joltage_jumps.get(&1).unwrap();
        let three_jumps = joltage_jumps.get(&3).unwrap() + 1;

        println!("Max Joltage {} 1 jumps : {} 3 jumps : {}", previous_number + 3, one_jumps, three_jumps);

        println!("1 jumps * 3 jumps = {}", one_jumps * three_jumps);


        let mut memo_array = Vec::new();

        numbers.push(previous_number + 3);

        for _i in 1..numbers.len() {
            memo_array.push(0);
        }
        memo_array.push(1);

        let mut pointer = numbers.len() - 1;

        println!(" {} {} {}", numbers.len(), memo_array.len(),pointer);

        loop {
            let i = pointer;
            let i_num = numbers.get(i).unwrap();
            for j in 1..4 {
                if i + j >= numbers.len() {
                    break;
                }
                let i_j_num = numbers.get(i + j).unwrap() + 0;

                for k in 1..4 {
                    if i_num + k == i_j_num {
                        memo_array.insert(i,memo_array.get(i).unwrap() + memo_array.get(i+j).unwrap());
                    } 
                }

            }
            if pointer == 0 {
                break;
            }
            pointer -= 1;
        }

        // for me in memo_array {
        //     println!("me : {}", me);
        // }

        println!("Paths at 0, {}", memo_array.get(0).unwrap());
    }
    else
    {
        println!("day_ten needs 2 args\n");
    }
}