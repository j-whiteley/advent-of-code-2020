use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    OCCUPIED,
    EMPTY,
    FLOOR
}

fn create_cell_table_from_string(input: &String) -> Vec<Vec<Cell>> {

    let mut cell_table = Vec::new();

    for row in input.split("\n") {
        let mut cell_row = Vec::new();

        for cell_char in row.chars() {
            match cell_char {
                'L' => cell_row.push(Cell::EMPTY),
                _ => cell_row.push(Cell::FLOOR)
            }
        }

        cell_table.push(cell_row);
    }

    return cell_table;
}

fn run_generation(input_table: Vec<Vec<Cell>>) -> (Vec<Vec<Cell>>, bool) {
    let mut output_table = Vec::new();
    let mut stable = true;

    for (i, input_row) in input_table.iter().enumerate() {
        let mut output_row = Vec::new();

        for (j, input_cell) in input_row.iter().enumerate() {
            let mut occupied_count = 0;
            for i_offset in -1..2 {
                let i_adj = i_offset + i as isize;
                if i_adj >= 0 && i_adj < input_table.len() as isize {
                    //valid i_adj
                    for j_offset in -1..2 {
                        let j_adj = j_offset + j as isize;
                        if j_adj >= 0 && j_adj < input_row.len() as isize {
                            let i_adj_u = i_adj as usize;
                            let j_adj_u = j_adj as usize;
                            //valid j_adj

                            if (i_offset != 0) || (j_offset !=0) {
                                // println!("({},{}) checking ({},{})", i, j, i_adj, j_adj);
                                if input_table[i_adj_u][j_adj_u] == Cell::OCCUPIED {
                                    occupied_count += 1;
                                }
                            }
                        }
                    }
                }
            }
            match input_cell {
                Cell::EMPTY => {
                    if occupied_count > 0 {
                        output_row.push(Cell::EMPTY);
                    } else {
                        stable = false;
                        output_row.push(Cell::OCCUPIED);
                    }
                },
                Cell::OCCUPIED => {
                    if occupied_count >= 4 {
                        stable = false;
                        output_row.push(Cell::EMPTY);
                    } else {
                        output_row.push(Cell::OCCUPIED);
                    }
                },
                Cell::FLOOR => output_row.push(Cell::FLOOR)
            }
        }

        output_table.push(output_row);
    }

    return (output_table, stable);
}

fn print_table(input_table: &Vec<Vec<Cell>>) {
    for input_row in input_table.iter() {
        for input_cell in input_row.iter() {
            match input_cell {
                Cell::OCCUPIED => {print!("#")},
                Cell::EMPTY => {print!("L")},
                Cell::FLOOR => {print!(".")}
            }
        }
        println!("");
    }
    println!("");
}

pub fn day_eleven(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let mut current_table = create_cell_table_from_string(&contents);

        let mut stable = false;

        while !stable {
            let (current_table_a, stable_a) = run_generation(current_table);
            current_table = current_table_a;
            stable = stable_a;

            // print_table(&current_table);
        }

        let mut occupied_count = 0;

        for current_row in current_table.iter() {
            for current_cell in current_row.iter() {
                if *current_cell == Cell::OCCUPIED {
                    occupied_count += 1;
                }
            }
        }

        println!("occ count: {}", occupied_count);

    }
    else
    {
        println!("day_eleven needs 2 args\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cell_table() {
        let cell_table = create_cell_table_from_string(&"L..\n.L.\n..L".to_string());

        assert_eq!(cell_table.len(),3);
        assert_eq!(cell_table[0].len(),3);
        assert_eq!(cell_table[0][0], Cell::EMPTY);
        assert_ne!(cell_table[0][0], Cell::FLOOR);
        assert_eq!(cell_table[0][1], Cell::FLOOR);
    }

    #[test]
    fn test_run_generation() {
        let cell_table = create_cell_table_from_string(&"L..\n.L.\n..L".to_string());

        let (next_table,stable) = run_generation(cell_table);

        print_table(&next_table);

        assert_eq!(next_table.len(),3);
        assert_eq!(next_table[0].len(),3);
        assert_eq!(next_table[0][0], Cell::OCCUPIED);
        assert_ne!(next_table[0][0], Cell::FLOOR);
        assert_eq!(next_table[0][1], Cell::FLOOR);
        assert!(!stable);
    }

    #[test]
    fn test_two_gens() {
        let cell_table = create_cell_table_from_string(&"LLL\nLLL\nLLL".to_string());
        print_table(&cell_table);

        let (next_table,_stable) = run_generation(cell_table);
        print_table(&next_table);
        let (next_table_2,_stable_2) = run_generation(next_table);
        print_table(&next_table_2);

    }
    
}