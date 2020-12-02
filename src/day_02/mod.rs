use std::fs;


struct PasswordPolicy {
    minimum: usize,
    maximum: usize,
    policy_char: String,
    password: String
}

fn parse_password_policy(input: String) -> PasswordPolicy {

    let parts = input.split(": ").collect::<Vec<&str>>();
    let my_password = parts[1];
    let policy_parts = parts[0].split(" ").collect::<Vec<&str>>();
    let number_parts = policy_parts[0].split("-").collect::<Vec<&str>>();
    let my_minimum = number_parts[0].parse::<usize>().unwrap();
    let my_maximum = number_parts[1].parse::<usize>().unwrap();

    PasswordPolicy {
        minimum: my_minimum,
        maximum: my_maximum,
        policy_char: policy_parts[1].to_string(),
        password: my_password.to_string()
    }
}

fn does_password_match_policy(policy: &PasswordPolicy) -> bool {
    let count = policy.password.matches(&policy.policy_char).count();

    return count >= policy.minimum && count <= policy.maximum;
}

fn does_password_match_other_policy(policy: &PasswordPolicy) -> bool {
    let first_char = &policy.password[(policy.minimum-1)..policy.minimum];

    let second_char = &policy.password[(policy.maximum-1)..policy.maximum];

    let first_char_matches = first_char == policy.policy_char;
    let second_char_matches = second_char == policy.policy_char;

    return first_char_matches != second_char_matches;
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
        let mut valid_counter_other = 0;

        for line in split {
            let line_policy = parse_password_policy(line.to_string());
            if does_password_match_policy(&line_policy) {
                valid_counter += 1;
            }

            if does_password_match_other_policy(&line_policy) {
                valid_counter_other += 1;
            }
        }

        println!("Valid Passwords {}\n", valid_counter);
        println!("Other Valid Passwords {}\n", valid_counter_other);

    }
    else
    {
        println!("day_two needs 2 args\n");
    }
}