use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let fd = File::open("input.txt")?;
    let reader = BufReader::new(fd);

    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();

    let mul: Vec<String> = reader
        .lines()
        .filter_map(|s| s.ok())
        .flat_map(|s| {
            re.find_iter(&s)
                .map(|m| m.as_str().to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    println!("{:?}", mul);

    Ok(())
}
