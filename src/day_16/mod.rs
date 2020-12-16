use std::fs;
use std::collections::HashSet;

struct Range {
    name: String,
    lower_start: usize,
    lower_end: usize,
    upper_start: usize,
    upper_end: usize
}

impl Range {
    fn new(input : &String) -> Range{
        let parts : Vec<String> = input.split(": ").map(|x| x.to_string()).collect();
        let name_part = parts[0].clone();
        let range_parts : Vec<String> = parts[1].split(" or ").map(|x| x.to_string()).collect();

        let lower_parts : Vec<usize> = range_parts[0].split("-").map(|x| x.to_string()).map(|x| x.parse::<usize>().unwrap()).collect();
        let upper_parts : Vec<usize> = range_parts[1].split("-").map(|x| x.to_string()).map(|x| x.parse::<usize>().unwrap()).collect();

        return Range {
            name: name_part,
            lower_start: lower_parts[0],
            lower_end: lower_parts[1],
            upper_start: upper_parts[0],
            upper_end: upper_parts[1],
        }
    }

    fn is_in_range(&self, number: usize) -> bool {
        return (number >= self.lower_start && number <= self.lower_end) || (number >= self.upper_start && number <= self.upper_end);
    }
}

pub fn day_sixteen(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split : Vec<String> = contents.split("\n\n").map(|x| x.to_string()).collect();

        println!("split length : {}", split.len());

        let mut ranges = Vec::new();

        for range_row in split[0].split("\n") {
            let range = Range::new(&range_row.to_string());
            ranges.push(range);
        }

        let my_numbers_section : Vec<String> = split[1].split("\n").map(|x| x.to_string()).collect();

        let my_numbers : Vec<usize> = my_numbers_section[1].split(",").map(|x| x.to_string()).map(|x| x.parse::<usize>().unwrap()).collect();

        let mut column_sets = Vec::new();

        for _ in my_numbers.iter() {
            let mut current_col_set = HashSet::new();

            for range in ranges.iter() {
                current_col_set.insert(range.name.clone());
            }

            column_sets.push(current_col_set);
        }

        let nearby_numbers_section : Vec<String> = split[2].split("\n").map(|x| x.to_string()).collect();

        let mut nearby_numbers = Vec::new();

        for i in 1..nearby_numbers_section.len() {
            let row_numbs : Vec<usize> = nearby_numbers_section[i].split(",").map(|x| x.to_string()).map(|x| x.parse::<usize>().unwrap()).collect();
            // println!("{} -  {}",i, row_numbs.len());
            nearby_numbers.push(row_numbs);
        }

        let mut error_count = 0;

        for row_num in nearby_numbers.iter() {
            let mut is_row_valid = true;
            for (_i,num) in row_num.iter().enumerate() {
                // println!("{}",*num);
                let mut num_in_any_range = false;
                for range in ranges.iter() {
                    if range.is_in_range(*num) {
                        num_in_any_range = true;
                        break;
                    } 
                    // else {
                    //     column_sets[i].remove(&range.name);
                    //     println!("Removing {} from {}", i, &range.name);
                    // }
                }

                if !num_in_any_range {
                    error_count += *num;
                    is_row_valid = false;
                }
            }

            if is_row_valid {
                for (i,num) in row_num.iter().enumerate() {
                    for range in ranges.iter() {
                        if !range.is_in_range(*num) {
                            column_sets[i].remove(&range.name);
                        } 
                    }
                }
            }
        }

        println!("Part one: {}", error_count);

        // let mut some_columns_have_move_than = true;

        // while some_columns_have_move_than {
        //     some_columns_have_move_than = false;

        //     for (i,column_set) in column_sets.iter().enumerate() {
        //         if column_set.len() == 1 {
        //             for value in column_set.iter() {
        //                 for (j,other_column_set) in column_sets.iter().enumerate() {
        //                     if i != j {
        //                         other_column_set.remove(value);
        //                     }
        //                 }
        //             } 

        //         } else if column_set.len() > 1{
        //             some_columns_have_move_than = true;
        //         }
        //     }
        // }

        // CBA doing all this set removal arse in Rust so doing by hand

        for (i,column_set) in column_sets.iter().enumerate() {
            println!("{}",i);
            for j in column_set.iter() {
                println!("Col {} is {}", i,j);
            }
        }

    }
    else
    {
        println!("day_sixteen needs 2 args\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_range() {
        let test_range = Range::new(&"class: 1-3 or 5-7".to_string());

        assert_eq!(test_range.name, "class".to_string());
        assert_eq!(test_range.lower_start, 1);
        assert_eq!(test_range.lower_end, 3);
        assert_eq!(test_range.upper_start, 5);
        assert_eq!(test_range.upper_end, 7);

    }

    #[test]
    fn test_in_range() {
        let test_range = Range::new(&"class: 1-3 or 5-7".to_string());

        assert!(!test_range.is_in_range(0));
        assert!(test_range.is_in_range(1));
        assert!(test_range.is_in_range(2));
        assert!(test_range.is_in_range(3));
        assert!(!test_range.is_in_range(4));
        assert!(test_range.is_in_range(5));
        assert!(test_range.is_in_range(6));
        assert!(test_range.is_in_range(7));
        assert!(!test_range.is_in_range(8));
    }
}