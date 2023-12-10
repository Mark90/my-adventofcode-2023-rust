use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/* https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html */
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_path(day: &'static str, test: bool) -> String {
    if !test {
        return format!("./inputs/{}.txt", day);
    } else {
        return format!("./inputs/{}.sample.txt", day);
    }
}
