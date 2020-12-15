use std::collections::HashMap;

fn play_crappy_game(starting_numbers : Vec<usize>, target: usize) -> usize {
    let mut previously_seen : HashMap<usize,usize> = HashMap::new();
    let mut last_number = starting_numbers[starting_numbers.len()-1];

    for i in 0..starting_numbers.len() - 1 {
        previously_seen.insert(starting_numbers[i], i);

    }

    for i in starting_numbers.len()..target {

        if previously_seen.contains_key(&last_number) {
            let next_last_number = i - *previously_seen.get(&last_number).unwrap() -1;

            previously_seen.insert(last_number, i -1);
            last_number = next_last_number;
        } else {
            previously_seen.insert(last_number, i-1);
            last_number = 0;
        }

       
    }

    return last_number;
}

pub fn day_fifteen(args: &Vec<String>) {

    if args.len() == 4
    {
        let starting_numbers: Vec<usize> = args[2].split(",").map(|x| x.to_string().parse::<usize>().unwrap()).collect();

        let target_number = args[3].parse::<usize>().unwrap();

        println!("Len of input {}, Target: {}", starting_numbers.len(), target_number);

        let result = play_crappy_game(starting_numbers, target_number);

        println!("Result - {}", result);

    }
    else
    {
        println!("day_fifteen needs 3 args\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_crappy_game() {
        let mut input = Vec::new();

        input.push(0);
        input.push(3);
        input.push(6);

        let output = play_crappy_game(input, 10);

        assert_eq!(output,0);
    }

    #[test]
    fn test_play_crappy_game_2() {
        let mut input = Vec::new();

        input.push(3);
        input.push(1);
        input.push(2);

        let output = play_crappy_game(input, 2020);

        assert_eq!(output,1836);
    }
}