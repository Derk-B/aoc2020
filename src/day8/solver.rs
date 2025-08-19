use crate::shared::file_reader;

type Instruction = (String, char, i32);
fn generate_mutatinos(vec: &Vec<Instruction>, idx: usize) -> Vec<Vec<Instruction>> {
    let mut res = vec.clone();
    // let mut elem = res.get(idx).unwrap();
    let (instr, operator, num) = res.get(idx).unwrap();
    println!("{:?}", vec.get(idx).unwrap());
    match instr.as_str() {
        "nop" => {
            let elem = (String::from("jmp"), operator.clone(), num.clone());
            res[idx] = elem;
        }
        "jmp" => {
            let elem = (String::from("nop"), operator.clone(), num.clone());
            res[idx] = elem;
        }
        _ => (),
    }
    // res[idx] = elem.clone();

    println!("{:?}", res.get(idx).unwrap());
    if idx >= vec.len() - 1 {
        return Vec::from([res]);
    } else {
        let mut result = generate_mutatinos(vec, idx + 1);
        result.push(res);
        return result;
    }
}

pub fn solve() {
    let lines = match file_reader::read_lines("files/day8/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let mut instructions: Vec<Instruction> = Vec::new();

    for line in lines.map_while(Result::ok) {
        let mut split = line.split(' ');
        let instr = split.next().unwrap();
        let remainder = split.next().unwrap();
        let operator = remainder.chars().next().unwrap();
        let num = remainder[1..].parse::<i32>().unwrap();

        instructions.push((instr.to_string(), operator, num));
    }

    let mut acc: i32 = 0;
    let mut pointer: usize = 0;
    let mut visited: Vec<usize> = Vec::new();
    while !visited.contains(&pointer) {
        let (instr, operator, num) = instructions.iter().nth(pointer).unwrap();
        visited.push(pointer);
        match instr.as_str() {
            "acc" => {
                if *operator == '+' {
                    acc += num;
                } else {
                    acc -= num;
                }
                pointer += 1;
            }
            "jmp" => {
                if *operator == '+' {
                    pointer += *num as usize;
                } else {
                    pointer -= *num as usize;
                }
            }
            _ => {
                pointer += 1;
            }
        }
    }

    println!("Day 8 - part 1: {}", acc);

    let mutations = generate_mutatinos(&instructions, 0);
    for list in mutations {
        // println!("Equality: {}", list == instructions);
        let mut acc: i32 = 0;
        let mut pointer: usize = 0;
        let mut visited: Vec<usize> = Vec::new();
        while !visited.contains(&pointer) && pointer < list.len() {
            // println!("{:?}, unwrapped: {:?}", list, list.get(pointer).unwrap());
            let (instr, operator, num) = list.get(pointer).unwrap();
            visited.push(pointer);
            match instr.as_str() {
                "acc" => {
                    if *operator == '+' {
                        acc += num;
                    } else {
                        acc -= num;
                    }
                    pointer += 1;
                }
                "jmp" => {
                    if *operator == '+' {
                        pointer += *num as usize;
                    } else {
                        pointer -= *num as usize;
                    }
                }
                _ => {
                    pointer += 1;
                }
            }
        }

        // println!("POinter: {}", pointer);
        if pointer >= list.len() - 1 {
            println!("Day 8 - part 2: {}", acc);
            break;
        }
    }
}
