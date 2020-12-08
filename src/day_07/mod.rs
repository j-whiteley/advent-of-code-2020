use std::fs;
use std::collections::HashMap;

#[derive(Debug,Clone)]
struct BagSpec {
    bag_name: String,
    can_contain_shiny_gold: bool,
    has_been_traversed: bool,
    bags_and_numbers: HashMap<String,usize>
}

impl BagSpec {
    fn new(input:&String) -> BagSpec {
        let first_split = input.split(" bags contain ").collect::<Vec<&str>>();
        let mut my_bags_numbers = HashMap::new();
        let mut this_can_contain_shiny_gold = false;

        let contains_parts = first_split[1];

        if !contains_parts.contains("no other") {
            for contains_split in contains_parts.split(", ") {
                let bag_split = contains_split.split(" ").collect::<Vec<&str>>();
                let number = bag_split[0].parse::<usize>().unwrap();
                let that_bag_name = format!("{} {}", bag_split[1], bag_split[2]);

                if that_bag_name.contains("shiny gold") {
                    this_can_contain_shiny_gold = true;
                }

                my_bags_numbers.insert(that_bag_name, number);
            }
        }

        return BagSpec {
            bag_name:first_split[0].to_string(),
            can_contain_shiny_gold:this_can_contain_shiny_gold,
            has_been_traversed:this_can_contain_shiny_gold,
            bags_and_numbers: my_bags_numbers
        };
    }

    fn set_has_been_traversed(&mut self, has_been_traversed: bool){
        self.has_been_traversed = has_been_traversed;
    }

    fn set_can_contain_shiny_gold(&mut self, can_contain_shiny_gold: bool){
        self.can_contain_shiny_gold = can_contain_shiny_gold;
    }
}

fn rec_contain_shiny_gold(key: String, map: &mut HashMap<String,BagSpec>) -> bool {
    let current_bag = map.get_mut(&key).unwrap().clone();
    let mut shiny_gold = current_bag.can_contain_shiny_gold;

    if !current_bag.has_been_traversed {
        for (this_key, _value) in current_bag.bags_and_numbers.iter() {
            let other = map.get_mut(this_key).unwrap();
            if !other.has_been_traversed {
                shiny_gold |= rec_contain_shiny_gold(this_key.to_string(), map);
            } else {
                shiny_gold |= other.can_contain_shiny_gold;
            }
        }
    }

    let current_bag_two = map.get_mut(&key).unwrap();

    current_bag_two.set_has_been_traversed(true);
    current_bag_two.set_can_contain_shiny_gold(shiny_gold);

    return shiny_gold;
}

fn rec_count_bags(key: String, map: &mut HashMap<String,BagSpec>) -> usize {
    let current_bag = map.get_mut(&key).unwrap().clone();
    let mut count = 1;

    if current_bag.bags_and_numbers.len() != 0 {
        for (key, value) in current_bag.bags_and_numbers.iter() {
            count += rec_count_bags(key.clone(), map) * value;
        }
    } else {
        count = 1;
    }
 
    return count;
}

pub fn day_seven(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split = contents.split("\n");
        let mut bag_map = HashMap::new();
        let mut key_vec = Vec::new();

        for line in split {
            let line_bag = BagSpec::new(&line.to_string());

            key_vec.push(line_bag.bag_name.clone());
            bag_map.insert(line_bag.bag_name.clone(), line_bag);
        }

        println!("{} Bag specs \n", bag_map.len());
        let mut count = 0;

        for key in key_vec {
            if rec_contain_shiny_gold(key.to_string(),&mut bag_map) {
                count += 1;
            }
        }
        println!("Count {}", count);

        println!("Shiny bag count : {} ", rec_count_bags("shiny gold".to_string(), &mut bag_map) - 1);
        
    }
    else
    {
        println!("day_seven needs 2 args\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bagspec() {
        let test_bag = BagSpec::new(&"bright white bags contain 1 shiny gold bag.".to_string());

        assert_eq!(test_bag.bag_name,"bright white");
        assert!(test_bag.bags_and_numbers.contains_key("shiny gold"));
        let number = test_bag.bags_and_numbers.get("shiny gold").unwrap();
        assert_eq!(*number,1);

        assert!(test_bag.can_contain_shiny_gold);
        assert!(test_bag.has_been_traversed);
    }

    #[test]
    fn test_new_bagspec_multi() {
        let test_bag = BagSpec::new(&"vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string());

        assert_eq!(test_bag.bag_name,"vibrant plum");
        assert!(test_bag.bags_and_numbers.contains_key("faded blue"));
        let number = test_bag.bags_and_numbers.get("faded blue").unwrap();
        assert_eq!(*number,5);

        assert!(test_bag.bags_and_numbers.contains_key("dotted black"));
        let number = test_bag.bags_and_numbers.get("dotted black").unwrap();
        assert_eq!(*number,6);

        assert!(!test_bag.can_contain_shiny_gold);
        assert!(!test_bag.has_been_traversed);
    }

    #[test]
    fn test_new_bagspec_nothing() {
        let test_bag = BagSpec::new(&"dotted black bags contain no other bags.".to_string());

        assert_eq!(test_bag.bag_name,"dotted black");
        
        assert_eq!(test_bag.bags_and_numbers.len(),0);

        assert!(!test_bag.can_contain_shiny_gold);
        assert!(!test_bag.has_been_traversed);
    }

}