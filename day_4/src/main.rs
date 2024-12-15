use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let fd = File::open("input.txt")?;
    let reader = BufReader::new(fd);

    let input_matrix: Vec<Vec<char>> = reader
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let keyword = "XMAS";

    let horizontal_sweep: Vec<usize> = input_matrix
        .clone()
        .into_iter()
        .map(|s| {
            let forward_pass = s.windows(4).map(|w| w.iter().collect::<String>());
            let backward_pass = s.windows(4).map(|w| w.iter().rev().collect::<String>());

            let fp_count = forward_pass.filter(|w| w == keyword).count();
            let bp_count = backward_pass.filter(|w| w == keyword).count();

            fp_count + bp_count
        })
        .collect();

    // println!("{:?}", input_matrix);
    println!("{:?}", horizontal_sweep);

    Ok(())
}
