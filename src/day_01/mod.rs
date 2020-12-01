use std::fs;

pub fn day_one(args: &Vec<String>) {

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

        for i in 0..numbers.len() {
            let mut found = false;
            for j in 1..numbers.len() {
                if numbers[i] + numbers[j] == 2020 {
                    let answer = numbers[i] * numbers[j];
                    println!(" {}  + {} = 2020, .'. Multiplied =  {}", numbers[i], numbers[j], answer);
                    
                    found = true;
                }
                if found {
                    break;
                }
            }

            if found {
                break;
            }
        }

        for i in 0..numbers.len() {
            let mut found = false;
            for j in 1..numbers.len() {
                for k in 2..numbers.len() {
                    if numbers[i] + numbers[j] + numbers[k] == 2020 {
                        let answer = numbers[i] * numbers[j] * numbers[k];
                        println!(" {}  + {} + {} = 2020, .'. Multiplied =  {}", numbers[i], numbers[j], numbers[k], answer);

                        found = true;
                    
                    }
                    if found {
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            if found {
                break;
            }
        }



    }
    else
    {
        println!("day_one needs 2 args\n");
    }
}