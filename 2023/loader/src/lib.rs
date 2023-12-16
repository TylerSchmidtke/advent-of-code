use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn load_day(day: &str) -> BufReader<File> {
    let day = format!("day_{}/input.txt", day);
    let path = Path::new(&day);
    let file = File::open(path).unwrap();
    BufReader::new(file)
}