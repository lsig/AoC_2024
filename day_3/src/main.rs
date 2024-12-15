use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug)]
enum Command {
    Mult(i64, i64),
    Do,
    Dont,
}

fn main() -> io::Result<()> {
    let fd = File::open("input.txt")?;
    let reader = BufReader::new(fd);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mul: i64 = reader
        .lines()
        .filter_map(|s| s.ok())
        .flat_map(|s| {
            re.captures_iter(&s)
                .filter_map(|c| {
                    let (_, [s1, s2]) = c.extract();
                    let n1 = s1.parse::<i64>().ok()?;
                    let n2 = s2.parse::<i64>().ok()?;

                    Some(n1 * n2)
                })
                .collect::<Vec<i64>>()
        })
        .sum();

    println!("{:?}", mul);

    let fd2 = File::open("input.txt")?;
    let reader2 = BufReader::new(fd2);

    let re2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do\(\))|(don't\(\))").unwrap();
    let mut do_mul = 1;

    let mul2: i64 = reader2
        .lines()
        .filter_map(|s| s.ok())
        .flat_map(|s| {
            re2.captures_iter(&s)
                .filter_map(|c| match (c.get(1), c.get(2), c.get(3), c.get(4)) {
                    (Some(s1), Some(s2), None, None) => Some(Command::Mult(
                        s1.as_str().parse::<i64>().ok()?,
                        s2.as_str().parse::<i64>().ok()?,
                    )),
                    (None, None, Some(_), None) => Some(Command::Do),
                    (None, None, None, Some(_)) => Some(Command::Dont),
                    _ => None,
                })
                .filter_map(|cmd| match cmd {
                    Command::Mult(n1, n2) => Some(n1 * n2 * do_mul),
                    Command::Do => {
                        do_mul = 1;
                        None
                    }
                    Command::Dont => {
                        do_mul = 0;
                        None
                    }
                })
                .collect::<Vec<i64>>()
        })
        .sum();

    println!("{:?}", mul2);

    Ok(())
}
