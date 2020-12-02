use std::fs;

struct Password_Policy {
    minimum: i32,
    maximum: i32,
    policy_char: char,
    password: String
}

fn parse_password_policy(input: &String) -> Password_Policy {
    println!("Parsing line: {}\n", input);

    Password_Policy {
        minimum: 0,
        maximum: 1,
        policy_char: 'a',
        password: String::from("password")
    }
}

pub fn day_two(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split = contents.split("\n");

        let mut valid_counter = 0;

        for line in split {
            parse_password_policy(&line);
        }

    }
    else
    {
        println!("day_two needs 2 args\n");
    }
}