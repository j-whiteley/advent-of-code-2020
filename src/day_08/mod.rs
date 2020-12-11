use std::fs;

struct Instruction {
    operation : String,
    numerator : isize
}

#[allow(unused_assignments)]
impl Instruction {
    fn new(input:&String) -> Instruction {
        let parts = input.split(" ").collect::<Vec<&str>>();

        let operator = &parts[1].to_string()[0..1];
        let number_wrap = &parts[1].to_string()[1..].parse::<isize>().unwrap();
        let mut number = 0;

        match operator {
            "-" => number = 0 - *number_wrap,
            _ => number = *number_wrap
        }

        return Instruction {
            operation: parts[0].to_string(),
            numerator: number
        }
    }
}

fn run_prog(instruction_set: &Vec<Instruction>, change_value: usize) -> (bool,usize) {


    let mut execution_tracker =  Vec::new();

    for _ in instruction_set.iter() {
        execution_tracker.push(false);
    }
    let mut program_counter = 0;
    let mut accumulator = 0;
    let mut execution_finished = false;

    loop {

        if program_counter >= instruction_set.len() {
            execution_finished = true;
            break;
        }

        if execution_tracker[program_counter] {
            break;
        }

        execution_tracker[program_counter] = true;

        let current_instruction = &instruction_set[program_counter];

        match &*current_instruction.operation {
            "jmp" => {
                if program_counter == change_value {
                    program_counter += 1;
                    //No-op
                }
                else {
                    if current_instruction.numerator > 0 {
                        program_counter += current_instruction.numerator as usize;
                    } else {
                        program_counter -= current_instruction.numerator.abs() as usize;
                    }
                }
            },
            "acc" => {

                if current_instruction.numerator > 0 {
                    accumulator += current_instruction.numerator as usize;
                } else {
                    accumulator -= current_instruction.numerator.abs() as usize;
                }
                program_counter += 1;
            },
            _ => {
                if program_counter == change_value {
                    //Jmp
                    if current_instruction.numerator > 0 {
                        program_counter += current_instruction.numerator as usize;
                    } else {
                        program_counter -= current_instruction.numerator.abs() as usize;
                    }
                }
                else {
                    program_counter += 1;
                }
            }
        }

    }

    return (execution_finished, accumulator);
}

pub fn day_eight(args: &Vec<String>) {

    if args.len() == 3
    {
        let filename = &args[2];
        println!("file name, {}\n", filename);

        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
    
        let split = contents.split("\n");

        let mut instruction_set = Vec::new();
        let mut execution_tracker =  Vec::new();

        for line in split {
            let instr = Instruction::new(&line.to_string());
            instruction_set.push(instr);
            execution_tracker.push(false);
        }

        println!("Instructions {}", instruction_set.len());
        let mut program_counter = 0;
        let mut accumulator = 0;
        let mut change_candidate = Vec::new();


        loop {
            if execution_tracker[program_counter] || program_counter > instruction_set.len() {
                break;
            }

            execution_tracker[program_counter] = true;

            let current_instruction = &instruction_set[program_counter];

            // println!("PC : {} ACC: {} Executing instr: {} {}", program_counter, accumulator, current_instruction.operation, current_instruction.numerator); 

            match &*current_instruction.operation {
                "jmp" => {
                    change_candidate.push(program_counter);
                    if current_instruction.numerator > 0 {
                        program_counter += current_instruction.numerator as usize;
                    } else {
                        program_counter -= current_instruction.numerator.abs() as usize;
                    }
                },
                "acc" => {

                    if current_instruction.numerator > 0 {
                        accumulator += current_instruction.numerator as usize;
                    } else {
                        accumulator -= current_instruction.numerator.abs() as usize;
                    }
                    program_counter += 1;
                },
                _ => {
                    change_candidate.push(program_counter);
                    program_counter += 1;
                }
            }

        }

        println!("Acc at break :{}", accumulator);

        for cand in change_candidate.iter() {
            // println!("{}",cand);
            let (success, value) = run_prog(&instruction_set, *cand);
            if success {
                println!("Cand {} right, Acc:{}",cand, value);
            }
        }
    }
    else
    {
        println!("day_eight needs 2 args\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_instruction() {
        let instruct = Instruction::new(&"nop +0".to_string());

        assert_eq!(instruct.operation,"nop");
        assert_eq!(instruct.numerator,0);
    }

    #[test]
    fn test_new_instruction_pos_num() {
        let instruct = Instruction::new(&"jmp +5".to_string());

        assert_eq!(instruct.operation,"jmp");
        assert_eq!(instruct.numerator,5);
    }

    #[test]
    fn test_new_instruction_neg_num() {
        let instruct = Instruction::new(&"acc -6".to_string());

        assert_eq!(instruct.operation,"acc");
        assert_eq!(instruct.numerator,-6);
    }
}