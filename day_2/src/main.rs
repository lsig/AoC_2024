use std::{
    fs::File,
    i64,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let fd = File::open("input.txt")?;
    let reader = BufReader::new(fd);

    let reports: Vec<Vec<i64>> = reader
        .lines()
        .filter_map(Result::ok)
        .map(|s| {
            s.split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect()
        })
        .collect();

    let safe = reports
        .iter()
        .map(|report| {
            report
                .windows(2)
                .all(|w| w[0] <= w[1] && i64::abs(w[1] - w[0]) >= 1 && i64::abs(w[1] - w[0]) <= 3)
                || report.windows(2).all(|w| {
                    w[0] >= w[1] && i64::abs(w[1] - w[0]) >= 1 && i64::abs(w[1] - w[0]) <= 3
                })
        })
        .filter(|&safe| safe)
        .count();
    // println!("{:?}", reports);
    println!("{:?}", safe);

    Ok(())
}
