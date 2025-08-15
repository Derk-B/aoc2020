use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn read_lines<T>(path: T) -> io::Result<io::Lines<io::BufReader<File>>>
where
    T: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}
