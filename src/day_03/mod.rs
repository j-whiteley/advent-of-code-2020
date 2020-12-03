use std::fs;

fn trees(hill: &Vec<&str>, x_increment: usize, y_increment: usize) -> usize {

    let mut x = 0;
    let mut y = 0;
    let mut treecounter = 0;

    for (index,level) in hill.iter().enumerate() {
        if index == y { 
            let cell = &level[x..x+1];

            match cell {
                "#" => treecounter+=1,
                _ => {}
            }

            x += x_increment;
            x = x % level.len();
            y += y_increment
        }
    }

    println!("Output x Inc: {}, y inc: {} is {} ", x_increment, y_increment, treecounter);

    return treecounter;

}

pub fn day_three(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let hill = contents.split("\n").collect::<Vec<&str>>();

        let first_trees = trees(&hill, 3, 1);

        println!("Tree count : {}", first_trees);

        let mut all_trees = trees(&hill, 1, 1);

        all_trees *= first_trees;

        all_trees *= trees(&hill, 5, 1);

        all_trees *= trees(&hill, 7, 1);

        all_trees *= trees(&hill, 1, 2);

        println!("All Trees {} ", all_trees);
        
    }
    else
    {
        println!("day_three needs 2 args\n");
    }
}