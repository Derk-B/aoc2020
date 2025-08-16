use crate::shared::file_reader;

fn merge_sort(list: &mut Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {
        return list.clone();
    }
    let div_idx: usize = list.len() / 2;

    let mut right: Vec<i32> = list.split_off(div_idx);

    let mut r = merge_sort(&mut right);
    let mut l = merge_sort(list);

    let mut res: Vec<i32> = Vec::new();
    while r.len() > 0 || l.len() > 0 {
        if r.len() == 0 {
            res.append(&mut l);
        } else if l.len() == 0 {
            res.append(&mut r);
        } else {
            if r.first().unwrap() < l.first().unwrap() {
                res.push(*r.first().unwrap());
                r.remove(0);
            } else {
                res.push(*l.first().unwrap());
                l.remove(0);
            }
        }
    }

    res
}

pub fn solve() {
    let lines = match file_reader::read_lines("files/day5/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let mut max: i32 = 0;
    let mut seat_ids: Vec<i32> = Vec::new();

    for line in lines.map_while(Result::ok) {
        if line.len() < 10 {
            continue;
        }
        let row_str = &line[..7];
        let col_str = &line[7..];

        let row = row_str
            .chars()
            .map(|c| match c {
                'F' => 0,
                _ => 1,
            })
            .fold(0, |acc, bit| (acc << 1) | bit as i32);

        let col = col_str
            .chars()
            .map(|c| match c {
                'L' => 0,
                _ => 1,
            })
            .fold(0, |acc, bit| (acc << 1) | bit as i32);

        let seat_id = row * 8 + col;
        if seat_id > max {
            max = seat_id;
        }
        seat_ids.push(seat_id);
    }
    println!("Day 5 - part 1: {}", max);

    seat_ids = merge_sort(&mut seat_ids);
    for i in 0..(seat_ids.len() - 1) {
        if seat_ids[i + 1] - seat_ids[i] == 2 {
            println!("Day 5 - part 2: {}", seat_ids[i] + 1);
            break;
        }
    }
}
