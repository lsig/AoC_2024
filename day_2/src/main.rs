use std::{
    fs::File,
    i64,
    io::{self, BufRead, BufReader},
};

fn make_combinations(slice: &[i64]) -> Vec<Vec<i64>> {
    let mut combinations: Vec<Vec<i64>> = (0..slice.len())
        .map(|i| {
            slice
                .iter()
                .take(i)
                .chain(slice.iter().skip(i + 1))
                .copied()
                .collect()
        })
        .collect();

    combinations.push(slice.to_vec());
    combinations
}

fn safety_check(slice: &[i64]) -> bool {
    slice
        .windows(2)
        .all(|w| w[0] < w[1] && i64::abs(w[1] - w[0]) >= 1 && i64::abs(w[1] - w[0]) <= 3)
        || slice
            .windows(2)
            .all(|w| w[0] > w[1] && i64::abs(w[1] - w[0]) >= 1 && i64::abs(w[1] - w[0]) <= 3)
}

fn main() -> io::Result<()> {
    let fd = File::open("input.txt")?;
    let reader = BufReader::new(fd);

    let reports: Vec<Vec<i64>> = reader
        .lines()
        .filter_map(|s| s.ok())
        .map(|s| {
            s.split_whitespace()
                .filter_map(|num| num.parse::<i64>().ok())
                .collect()
        })
        .collect();

    let safe = reports
        .iter()
        .filter(|report| safety_check(&report))
        .count();

    let safe2 = reports
        .iter()
        .map(|report| make_combinations(&report))
        .filter(|report| report.iter().any(|combination| safety_check(&combination)))
        .count();

    println!("{:?}", safe);
    println!("{:?}", safe2);

    Ok(())
}

#[test]
fn test_make_combinations() {
    let input = vec![1, 2, 3, 4];
    let mut result = make_combinations(&input);
    let mut test = vec![
        vec![1, 2, 3],
        vec![1, 3, 4],
        vec![2, 3, 4],
        vec![1, 2, 4],
        input,
    ];

    result.sort();
    test.sort();

    assert_eq!(result, test);
}
