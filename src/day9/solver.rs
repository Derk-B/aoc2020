use crate::shared::file_reader;

fn range_contains_sum(range: &[u64], sum: &u64) -> bool {
    let window_size = range.len();
    for i in 0..window_size {
        for j in 0..window_size {
            if i == j {
                continue;
            }
            if range[i] + range[j] == *sum {
                return true;
            }
        }
    }

    false
}

pub fn solve() {
    let lines = match file_reader::read_lines("files/day9/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let mut numbers: Vec<u64> = Vec::new();

    for line in lines.map_while(Result::ok) {
        numbers.push(line.parse::<u64>().unwrap());
    }
    // 5 for test input, 25 for real input
    let window_size = 25;
    let mut found_value: u64 = 0;
    for i in window_size..numbers.len() {
        let current = &numbers[i];
        let window = &numbers[i - window_size..i];

        if !range_contains_sum(window, current) {
            println!("Day 9 - part 1: {}", current);
            found_value = *current;
            break;
        }
    }

    let mut idx: usize = 0;
    let mut sum: u64 = 0;
    let mut range: Vec<u64> = Vec::new();
    while idx < numbers.len() {
        let mut i: usize = idx;
        while i + idx < numbers.len() {
            if sum + numbers[idx + i] > found_value {
                // print!("{} > {}", sum + numbers[idx + i]);
                range = Vec::new();
                sum = 0;
                break;
            }
            println!("Next");
            if sum + numbers[idx + i] == found_value {
                range.push(numbers[i + idx]);
                let mut m1: u64 = range[0];
                let mut m2: u64 = range[0];
                for n in &range {
                    if *n < m1 {
                        m1 = *n;
                    }
                    if *n > m2 {
                        m2 = *n;
                    }
                }
                let mres = m1 + m2;
                println!("Day 9 - part 2: {} {} {}", m1, m2, mres);
                let total = range.iter().fold(0, |acc, n| acc + n);
                println!("Total: {}", total + numbers[idx + i]);
                break;
            }

            range.push(numbers[idx + i]);
            sum += numbers[idx + i];
            i += 1;
        }
        idx += 1;
        // println!("IDX: {}", idx);
    }
}

// 1187622051
// 1309761972
