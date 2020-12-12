use std::fs;

const COMPASS : &'static [Direction] = &[Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST];

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    LEFT,
    RIGHT,
    FORWARD
}

struct NavigationInstruction {
    direction:Direction,
    magnitude:isize
}

impl NavigationInstruction {
    fn new(input: &String) -> NavigationInstruction {
        let direction_char = input[0..1].to_string();
        let magnitude = input[1..].to_string().parse::<isize>().unwrap();

        let direction = match direction_char.as_str()  {
            "N" => Direction::NORTH,
            "S" => Direction::SOUTH,
            "W" => Direction::WEST,
            "E" => Direction::EAST,
            "F" => Direction::FORWARD,
            "L" => Direction::LEFT,
            "R" => Direction::RIGHT,
            _ => Direction::FORWARD
        };

        return NavigationInstruction {
            direction:direction,
            magnitude:magnitude
        }
    }
}

fn interpret_instructions(all_instructions: &Vec<NavigationInstruction>, starting_direction: isize) -> (isize,isize,isize) {

    let mut x : isize = 0;
    let mut y : isize = 0;
    let mut current_direction = starting_direction; 

    for instruction in all_instructions.iter() {
        match instruction.direction {
            Direction::NORTH => {y += instruction.magnitude;},
            Direction::EAST => {x += instruction.magnitude;},
            Direction::SOUTH => {y -= instruction.magnitude;},
            Direction::WEST => {x -= instruction.magnitude;},
            Direction::RIGHT => {
                let turns = instruction.magnitude / 90;
                current_direction = (current_direction + turns) % 4;
            },
            Direction::LEFT => {
                let turns = instruction.magnitude / 90;
                current_direction = current_direction - turns;
                if current_direction < 0 {
                    current_direction += 4;
                }
            },
            Direction::FORWARD => {
                match COMPASS[current_direction as usize] {
                    Direction::NORTH => {y += instruction.magnitude;},
                    Direction::EAST => {x += instruction.magnitude;},
                    Direction::SOUTH => {y -= instruction.magnitude;},
                    Direction::WEST => {x -= instruction.magnitude;},
                    _ => {}
                }
            }
        }
    }

    return (x,y,current_direction);
}

fn interpret_instructions_two(all_instructions: &Vec<NavigationInstruction>, start_waypoint_x : isize, start_waypoint_y: isize) -> (isize,isize) {
    let mut x : isize = 0;
    let mut y : isize = 0;
    let mut waypoint_x = start_waypoint_x; 
    let mut waypoint_y = start_waypoint_y;
    
    for instruction in all_instructions.iter() {
        match instruction.direction {
            Direction::NORTH => {waypoint_y += instruction.magnitude;},
            Direction::EAST => {waypoint_x += instruction.magnitude;},
            Direction::SOUTH => {waypoint_y -= instruction.magnitude;},
            Direction::WEST => {waypoint_x -= instruction.magnitude;},
            Direction::RIGHT => {
                let turns = instruction.magnitude / 90;
                for _i in 0..turns {
                    let temp_x = waypoint_x;
                    waypoint_x = waypoint_y;
                    waypoint_y = -temp_x;
                }
            },
            Direction::LEFT => {
                let turns = instruction.magnitude / 90;
                for _i in 0..turns {
                    let temp_x = waypoint_x;
                    waypoint_x = -waypoint_y;
                    waypoint_y = temp_x;
                }
            },
            Direction::FORWARD => {
                x += waypoint_x * instruction.magnitude;
                y += waypoint_y * instruction.magnitude;
            }
        }
    }

    return (x,y);
}

pub fn day_twelve(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split = contents.split("\n");

        let mut all_instructions = Vec::new();
        for row in split {
            all_instructions.push(NavigationInstruction::new(&row.to_string()));
        }

        println!("{} Navigation instructions", all_instructions.len());

        let (x,y, _direction) = interpret_instructions(&all_instructions, 1); 

        println!("({},{}) - manhattan {}", x,y,x.abs()+y.abs());

        let (x_2,y_2) = interpret_instructions_two(&all_instructions, 10 , 1);

        println!("({},{}) - manhattan {}", x_2,y_2,x_2.abs()+y_2.abs());

    }
    else
    {
        println!("day_twelve needs 2 args\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_nav_ins() {
        let nav_ins = NavigationInstruction::new(&"R90".to_string());

        assert_eq!(nav_ins.direction,Direction::RIGHT);
        assert_eq!(nav_ins.magnitude,90);
    }

    #[test]
    fn test_interpret_left() {
        let mut inst_vec = Vec::new();
        inst_vec.push(NavigationInstruction::new(&"L180".to_string()));

        let (x,y,direction) = interpret_instructions(&inst_vec,1);

        assert_eq!(x,0);
        assert_eq!(y,0);
        assert_eq!(direction,3);
    }

    #[test]
    fn test_interpret_right() {
        let mut inst_vec = Vec::new();
        inst_vec.push(NavigationInstruction::new(&"R180".to_string()));

        let (x,y,direction) = interpret_instructions(&inst_vec,1);

        assert_eq!(x,0);
        assert_eq!(y,0);
        assert_eq!(direction,3);
    }
}