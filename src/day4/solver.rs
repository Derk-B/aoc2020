use crate::shared::file_reader;

fn valid_year(value: &str, min: i32, max: i32) -> bool {
    let year = value.parse::<i32>().unwrap_or_default(); // Defaults to 0 which is an invalid
    // number in any case.
    year >= min && year <= max
}

fn valid_height(value: &str) -> bool {
    if value.len() < 3 {
        return false;
    }

    #[derive(Debug)]
    enum MeasureType {
        IN,
        CM,
        OTHER,
    }

    let t: MeasureType = match &value[value.len() - 2..] {
        "in" => MeasureType::IN,
        "cm" => MeasureType::CM,
        _ => MeasureType::OTHER,
    };

    if let MeasureType::OTHER = t {
        return false;
    }

    let n = value[..value.len() - 2].parse::<i32>().unwrap_or_default();

    if let MeasureType::IN = t {
        return n >= 59 && n <= 76;
    }

    if let MeasureType::CM = t {
        return n >= 150 && n <= 193;
    }

    false
}

fn valid_hair_color(value: &str) -> bool {
    if !value.starts_with("#") {
        return false;
    }

    if value.len() > 7 {
        return false;
    }

    // Each character must be 0-9 or a-f, which is the same as a base-16 digit.
    if value[1..].to_string().chars().all(|c| c.is_digit(16)) {
        return true;
    }

    false
}

fn valid_eye_color(value: &str) -> bool {
    let value_colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    value_colors.iter().any(|col| col == &value)
}

fn valid_passport_id(value: &str) -> bool {
    if value.len() != 9 {
        return false;
    }

    if value.to_string().chars().all(|c| c.is_digit(10)) {
        return true;
    }

    false
}

pub fn solve() {
    let lines = match file_reader::read_lines("files/day4/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let mut passports: Vec<String> = Vec::new();
    let mut new_passport: String = String::new();
    for mut line in lines.map_while(Result::ok) {
        if line.is_empty() {
            passports.push(new_passport);
            new_passport = String::new();
        }
        line.insert_str(0, " ");
        new_passport += line.as_str();
    }
    passports.push(new_passport);

    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid_passports_part1: Vec<&String> = Vec::new();
    for p in &passports {
        if required_fields.iter().all(|field| p.contains(field)) {
            valid_passports_part1.push(p);
        }
    }

    println!("Day 4 - part 1: {}", valid_passports_part1.len());

    let mut valid_passport_count = 0;
    for p in valid_passports_part1 {
        let mut passport_valid = true;
        for field in p.split_whitespace() {
            let parts: Vec<&str> = field.split(":").collect();
            let key = parts[0];
            let value = parts[1];

            let valid = match key {
                "byr" => valid_year(value, 1920, 2002),
                "iyr" => valid_year(value, 2010, 2020),
                "eyr" => valid_year(value, 2020, 2030),
                "hgt" => valid_height(value),
                "hcl" => valid_hair_color(value),
                "ecl" => valid_eye_color(value),
                "pid" => valid_passport_id(value),
                _ => true,
            };

            if !valid {
                passport_valid = false;
                break;
            }
        }
        if passport_valid {
            valid_passport_count += 1;
        }
    }
    println!("Day 4 - part 2: {}", valid_passport_count);
}
