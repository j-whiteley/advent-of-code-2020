use std::fs;
use std::collections::VecDeque;

pub fn day_nine(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split = contents.split("\n");
        let mut numbers = VecDeque::new();
        let mut second_numbers = VecDeque::new();

        for row in split {
            let current_num = row.parse::<i64>().unwrap();
            numbers.push_back(current_num);
            second_numbers.push_back(current_num);
        }

        println!("Numbers : {}", numbers.len());

        let mut preamble = 25;

        if numbers.len() < 25 {
            preamble = 5;
        }

        println!("Preabme: {}", preamble);

        let mut body = numbers.split_off(preamble);
        let mut answer = 0;

        while let Some(current) = body.pop_front() {
            let mut found_yet = false;

            'toplevel: for i in 0..preamble {
                for j in i..preamble {
                    match(numbers.get(i), numbers.get(j)) {
                        (Some(k), Some(l)) => {
                            if k + l == current {
                                found_yet = true;
                                break 'toplevel;
                            }
                        },
                        _ => {}
                    }
                }
            }

            if !found_yet {
                answer = current;
            }

            numbers.pop_front();
            numbers.push_back(current);
        }

        println!("Answer: {}", answer);

        let mut current_chain = VecDeque::new();
        
        let mut current_sum = second_numbers.pop_front().unwrap();

        current_chain.push_back(current_sum);

        println!("Current sum : {}", current_sum);

        let mut sequence_found = false;

        while !sequence_found {
            if current_chain.len() < 2 {
                current_chain.push_back(second_numbers.pop_front().unwrap());
                continue;
            }

            let mut local_sum :i64 = 0;

            for local_value in current_chain.iter() {
                local_sum += local_value;
            }

            if local_sum < answer {
                current_chain.push_back(second_numbers.pop_front().unwrap());
            } else if local_sum > answer {
                current_chain.pop_front();
            } else if local_sum == answer {
                sequence_found = true;
            } else {
                panic!("unreachable");
            }
        }

        println!("current chain leng {}", current_chain.len());

        let mut min = current_chain.get(0).unwrap();
        let mut max = min;
        
        for local_value in current_chain.iter() {
            if local_value > max {
                max = local_value;
            }

            if local_value < min {
                min = local_value;
            }
        }

        println!("Max: {} Min: {} Product: {}", max, min, (max+min));


    }
    else
    {
        println!("day_nine needs 2 args\n");
    }
}