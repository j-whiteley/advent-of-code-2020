use std::fs;

fn parse_seat_id(seat_id_enc : String) -> isize {

    let row_enc = &seat_id_enc[0..7];
    let col_enc = &seat_id_enc[7..10];

    let row_bin = row_enc.replace("F","0").replace("B","1");

    let col_bin = col_enc.replace("R","1").replace("L","0");

    let row_num = isize::from_str_radix(&row_bin, 2).unwrap();
    let col_num = isize::from_str_radix(&col_bin, 2).unwrap();
    let seat_id = (row_num * 8) + col_num;
    
    return seat_id;
}

pub fn day_five(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split = contents.split("\n");

        let mut current_max :isize = 0;
        let mut current_min :isize = 1024;

        let mut occupied = Vec::new();

        for i in 0..1024 {
            occupied.push(true);
        }

        for row in split {
            let current_seat_id = parse_seat_id(row.to_string());

            if current_seat_id > current_max {
                current_max = current_seat_id;
            }

            if current_seat_id < current_min {
                current_min = current_seat_id;
            }

            occupied[current_seat_id as usize] = false;
        }

        let mut my_seat_id = 0;

        for i in current_min..current_max {
            if occupied[i as usize] {
                my_seat_id = i;
                break;
            }
        }

        println!("Max seat id: {}, Min seat id: {}, My seat ID: {}", current_max, current_min, my_seat_id);

    }
    else
    {
        println!("day_five needs 2 args\n");
    }
}