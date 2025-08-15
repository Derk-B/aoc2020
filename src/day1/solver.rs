use crate::shared::file_reader;
use std::collections::HashMap;

pub fn solve() {
    let lines = match file_reader::read_lines("files/day1/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let mut map: HashMap<i32, i32> = HashMap::new();
    let target: i32 = 2020;
    for line in lines.map_while(Result::ok) {
        let number = match line.parse::<i32>() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        };
        map.insert(number, target - number);
    }

    for (k, v) in map.iter() {
        if map.contains_key(v) {
            println!("Day 1 - part 1: {}", k * v);
            break;
        }
    }

    for k1 in map.keys() {
        for k2 in map.keys() {
            for k3 in map.keys() {
                if k1 + k2 + k3 == target {
                    println!("Day 1 - part 2: {}", k1 * k2 * k3);
                    return;
                }
            }
        }
    }

    println!("No result found!");
}
