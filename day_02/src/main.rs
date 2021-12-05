use std::fs;
use std::time::Instant;

type Generated = Vec<Vec<i32>>;

fn generate(input: &str) -> Generated {
    input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn part_1(input: &Generated) -> i32 {
    input
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

fn part_2(input: &Generated) -> i32 {
    input
        .iter()
        .map(|row| {
            let mut values = row
                .iter()
                .enumerate()
                .filter(|v| {
                    row.iter()
                        .enumerate()
                        .any(|rv| v.0 != rv.0 && (rv.1 % v.1 == 0 || v.1 % rv.1 == 0))
                })
                .map(|v| *v.1)
                .collect::<Vec<i32>>();
            values.sort();
            let val1 = values[0];
            let val2 = values[1];
            val2 / val1
        })
        .sum()
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");

    let data = generate(&content);

    let res1_start = Instant::now();
    let res1 = part_1(&data);
    let res1_stop = Instant::now();

    let res2_start = Instant::now();
    let res2 = part_2(&data);
    let res2_stop = Instant::now();

    print!(
        "Result1: {}\nResolved in: {:?}\n",
        res1,
        res1_stop.duration_since(res1_start)
    );
    print!(
        "Result2: {}\nResolved in: {:?}\n",
        res2,
        res2_stop.duration_since(res2_start)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(
            18,
            part_1(&generate(
                "5 1 9 5
7 5 3
2 4 6 8"
            ))
        );
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            9,
            part_2(&generate(
                "5 9 2 8
9 4 7 3
3 8 6 5"
            ))
        );
    }
}
