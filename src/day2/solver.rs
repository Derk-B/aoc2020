use crate::shared::file_reader;
use regex::Regex;

pub fn solve() {
    let lines = match file_reader::read_lines("files/day2/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let re = match Regex::new(r"([0-9]+)-([0-9]+)\s+([a-z]):\s+([a-z]+)") {
        Ok(r) => r,
        Err(e) => panic!("Failed to compile regex with error: {}", e),
    };

    let mut count_correct_passwords_part1: i32 = 0;
    let mut count_correct_passwords_part2 = 0;
    for line in lines.map_while(Result::ok) {
        let res = match re.captures(&line) {
            Some(r) => r,
            None => panic!("Failed to match on line: {}", line),
        };

        let left = res.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let right = res.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let char = res.get(3).unwrap().as_str().chars().next().unwrap();
        let password = res.get(4).unwrap().as_str();

        let mut count = 0;
        for c in password.chars() {
            if c == char {
                count += 1;
            }
        }
        if count >= left && count <= right {
            count_correct_passwords_part1 += 1;
        }

        if (password.chars().nth(left - 1).unwrap() == char)
            ^ (password.chars().nth(right - 1).unwrap() == char)
        {
            count_correct_passwords_part2 += 1;
        }
    }

    println!("Day 2 - part 1: {}", count_correct_passwords_part1);
    println!("Day 2 - part 2: {}", count_correct_passwords_part2);
}
