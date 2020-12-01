use std::fs;

pub fn day_zero(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        println!("With text:\n{}", contents);

    }
    else
    {
        println!("day_zero needs 2 args\n");
    }
}