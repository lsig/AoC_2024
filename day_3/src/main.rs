use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let fd = File::open("input.txt")?;
    let reader = BufReader::new(fd);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mul: i64 = reader
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| {
            re.captures_iter(&s)
                .filter_map(|c| {
                    let (_, [s1, s2]) = c.extract();
                    let n1 = s1.parse::<i64>().ok()?;
                    let n2 = s2.parse::<i64>().ok()?;

                    Some(n1 * n2)
                })
                .sum::<i64>()
        })
        .sum();

    println!("{:?}", mul);

    Ok(())
}
