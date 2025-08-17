use crate::shared::file_reader;

const N_QUESTIONS: usize = 26;

fn sum_all_answers(sum: &mut u32, question_arr: [u32; 26]) {
    *sum += question_arr.iter().fold(0, |acc, asked| {
        if *asked > 0 {
            return acc + 1;
        } else {
            return acc;
        }
    });
}

fn sum_common_answers(sum: &mut u32, question_arr: [u32; 26], group_size: u32) {
    *sum += question_arr.iter().fold(0, |acc, asked| {
        if *asked == group_size {
            return acc + 1;
        } else {
            return acc;
        }
    });
}

pub fn solve() {
    let lines = match file_reader::read_lines("files/day6/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let mut question_arr: [u32; N_QUESTIONS] = [0; N_QUESTIONS];

    let mut sum_part1: u32 = 0;
    let mut sum_part2: u32 = 0;
    let mut group_size: u32 = 0;
    for line in lines.map_while(Result::ok) {
        if line.is_empty() {
            sum_all_answers(&mut sum_part1, question_arr);
            sum_common_answers(&mut sum_part2, question_arr, group_size);
            question_arr = [0; N_QUESTIONS];
            group_size = 0;
        } else {
            group_size += 1;
            for c in line.chars() {
                question_arr[c as usize - 97] += 1;
            }
        }
    }
    sum_all_answers(&mut sum_part1, question_arr);
    sum_common_answers(&mut sum_part2, question_arr, group_size);

    println!("Day 6 - part 1: {}", sum_part1);
    println!("Day 6 - part 2: {}", sum_part2);
}
