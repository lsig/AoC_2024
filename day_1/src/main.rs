use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let fd = File::open("input.txt")?;
    let reader = io::BufReader::new(fd);

    let (mut list1, mut list2): (Vec<i64>, Vec<i64>) = reader
        .lines()
        .filter_map(Result::ok)
        .filter_map(|s| {
            let mut elements = s.split_whitespace();
            Some((
                elements.next()?.to_string().parse::<i64>().ok()?,
                elements.next()?.to_string().parse::<i64>().ok()?,
            ))
        })
        .unzip();

    list1.sort();
    list2.sort();

    // Part 1
    let difference: i64 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Difference: {:?}", difference);

    // Part 2
    let counter = list2.iter().fold(HashMap::new(), |mut map, item| {
        *map.entry(item).or_insert(0) += 1;
        map
    });

    let similarity: i64 = list1
        .iter()
        .map(|val| val * counter.get(val).unwrap_or(&0))
        .sum();

    println!("Similarity : {:?}", similarity);

    Ok(())
}
