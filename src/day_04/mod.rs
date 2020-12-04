use std::fs;
use std::collections::HashMap;

pub fn day_four(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        
        let split = contents.split("\n");

        let mut passports = Vec::new();
        let mut currentPassport = HashMap::new();

        for line in split {
            if line.len() < 3 {
                //Complete Pass port
                passports.push(currentPassport);
                currentPassport = HashMap::new();

            } else {
                println!("Parsing line {}", line);
                currentPassport.insert("abc".to_string(),"def".to_string());
            }
        }

        println!("Found {} passports\n", passports.len());
        
    }
    else
    {
        println!("day_four needs 2 args\n");
    }
}