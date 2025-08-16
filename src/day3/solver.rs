use crate::shared::file_reader;

struct Slope {
    dx: usize,
    dy: usize,
}

struct Map {
    field: Vec<String>,
    width: usize,
    height: usize,
}

impl Slope {
    fn traverse(&self, map: &Map) -> i32 {
        let mut y: usize = 0;
        let mut x: usize = 0;
        let mut tree_count = 0;
        while y < map.height {
            if map.field.iter().nth(y).unwrap().chars().nth(x).unwrap() == '#' {
                tree_count += 1;
            }
            y += self.dy;
            x += self.dx;
            x %= map.width;
        }

        tree_count
    }
}

impl Map {
    fn new(field: Vec<String>) -> Map {
        let h = field.len();
        let w = field.first().unwrap().len();
        Map {
            field,
            width: w,
            height: h,
        }
    }
}

pub fn solve() {
    let lines = match file_reader::read_lines("files/day3/input.txt") {
        Ok(ls) => ls,
        Err(e) => panic!("{}", e),
    };

    let mut field: Vec<String> = Vec::new();

    for line in lines.map_while(Result::ok) {
        field.push(line);
    }
    let map = Map::new(field);
    let slopes: Vec<Slope> = vec![
        Slope { dx: 3, dy: 1 },
        Slope { dx: 1, dy: 1 },
        Slope { dx: 5, dy: 1 },
        Slope { dx: 7, dy: 1 },
        Slope { dx: 1, dy: 2 },
    ];

    println!("Day 3 - part 1: {}", slopes.first().unwrap().traverse(&map));
    let mut tree_count_product: i32 = 1;
    for slope in slopes {
        tree_count_product = tree_count_product * slope.traverse(&map);
    }
    println!("Day 3 - part 2: {}", tree_count_product);
}
