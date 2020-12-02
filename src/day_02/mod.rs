use std::fs;


struct Password_Policy {
    minimum: usize,
    maximum: usize,
    policy_char: String,
    password: String
}

fn parse_password_policy(input: String) -> Password_Policy {

    let parts = input.split(": ").collect::<Vec<&str>>();
    let myPassword = parts[1];
    let policyParts = parts[0].split(" ").collect::<Vec<&str>>();
    let numberParts = policyParts[0].split("-").collect::<Vec<&str>>();
    let myMinimum = numberParts[0].parse::<usize>().unwrap();
    let myMaximum = numberParts[1].parse::<usize>().unwrap();

    Password_Policy {
        minimum: myMinimum,
        maximum: myMaximum,
        policy_char: policyParts[1].to_string(),
        password: myPassword.to_string()
    }
}

fn does_password_match_policy(policy: Password_Policy) -> bool {
    let count = policy.password.matches(&policy.policy_char).count();

    return (count >= policy.minimum && count <= policy.maximum);
}

fn does_password_match_other_policy(policy: Password_Policy) -> bool {
    let first_char = &policy.password[(policy.minimum-1)..policy.minimum];

    let second_char = &policy.password[(policy.maximum-1)..policy.maximum];

    let first_char_matches = (first_char == policy.policy_char);
    let second_char_matches = (second_char == policy.policy_char);

    // println!("Password : {}, first char {}-{}, second char {}-{}", policy.password, first_char, first_char_matches, second_char, second_char_matches);

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
            // if does_password_match_policy(line_policy) {
            //     valid_counter += 1;
            // }

            if does_password_match_other_policy(line_policy) {
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