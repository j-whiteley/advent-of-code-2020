use std::fs;

fn parse_input(input: &Vec<String>) -> (usize, Vec<usize>)  {
    let time = input[0].parse::<usize>().unwrap();
    let mut bus_ids = Vec::new();

    for bus_id in input[1].split(",") {
        if bus_id != &"x".to_string() {
            let bus_id_int = bus_id.parse::<usize>().unwrap();
            bus_ids.push(bus_id_int);
        }
    }

    return (time, bus_ids);
}

fn find_earliest_part_one(earliest_time : usize, bus_ids: Vec<usize>) -> usize {
    let mut current_bus_id = 1;
    // let mut current_multiplier = 1;
    let mut current_difference = earliest_time;

    for bus_id in bus_ids {
        let multiplier = (earliest_time / bus_id) + 1;
        let current_time = multiplier * bus_id;
        let this_difference = current_time - earliest_time;
        if this_difference < current_difference {
            current_bus_id = bus_id;
            // current_multiplier = multiplier;
            current_difference = this_difference;
        }
    }

    return current_difference * current_bus_id;
}

fn part_two(bus_line: String) -> i64 {
    let mut result : i64 = 0;
    let mut multiplier : i64 = 1;

    for (i,bus_id) in bus_line.split(",").enumerate() {
        if bus_id !=  &"x".to_string() {
            let current_bus_id = bus_id.parse::<i64>().unwrap();

            while (result + i as i64) % current_bus_id != 0 {
                result += multiplier;
            }

            multiplier *= current_bus_id;
        }
    }

    return result;
}


pub fn day_thirteen(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split: Vec<String> = contents.split("\n").map(|x| x.to_string()).collect();

        let (earliest_time, bus_ids) = parse_input(&split);

        let answer_part_one = find_earliest_part_one(earliest_time, bus_ids);

        println!("Part One : {}", answer_part_one);

        let answer_part_two = part_two(split[1].clone());

        println!("Part Two : {}", answer_part_two);

    }
    else
    {
        println!("day_thirteen needs 2 args\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let mut input = Vec::new();
        input.push("939".to_string());
        input.push("7,13,x,x,59,x,31,19".to_string());

        let (time, bus_ids) = parse_input(&input);

        assert_eq!(time,939);
        assert_eq!(bus_ids.len(),5);
        assert_eq!(bus_ids[0],7);
    }

    #[test]
    fn test_part_one() {
        let mut input = Vec::new();
        input.push("939".to_string());
        input.push("7,13,x,x,59,x,31,19".to_string());

        let (time, bus_ids) = parse_input(&input);

        let answer = find_earliest_part_one(time, bus_ids);

        assert_eq!(answer,295);

    }

    #[test]
    fn test_part_two() {
        let part_two_result = part_two("17,x,13,19".to_string());
        
        assert_eq!(part_two_result,3417);

        let part_two_result_1 = part_two("67,7,59,61".to_string());
        
        assert_eq!(part_two_result_1,754018);

        let part_two_result_2 = part_two("67,x,7,59,61".to_string());
        
        assert_eq!(part_two_result_2,779210);
    }
}