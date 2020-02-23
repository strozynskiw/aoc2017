use std::fs;

fn part_1(input: &str) -> u32 {
    let iter = input.chars().chain(input.chars().nth(0));
    let digits :Vec<u32> = iter.map(|c| c.to_digit(10).unwrap() as u32).collect();
    let mut sum :u32 = 0;
    for i in 0..digits.len()-1 {
        if digits[i] == digits[i+1] {
            sum += digits[i];
        }
    }
    sum
}

fn part_2(input: &str) -> u32 {
    let iter = input.chars().chain(input.chars().take(input.len()/2));
    let digits :Vec<u32> = iter.map(|c| c.to_digit(10).unwrap() as u32).collect();
    let mut sum :u32 = 0;
    for i in 0..input.len() {
        if digits[i] == digits[(input.len()/2)+i] {
            sum += digits[i];
        }
    }
    sum
}

fn main() {
    let content = fs::read_to_string("input").expect("file not found");
    let content = content.trim();
    print!("Result1: {}\n", part_1(&content));
    print!("Result2: {}\n", part_2(&content));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        assert_eq!(3, part_1("1122"));
        assert_eq!(4, part_1("1111"));
        assert_eq!(0, part_1("1234"));
        assert_eq!(9, part_1("91212129"));
    }
    #[test]
    fn test_part_2() {
        assert_eq!(6, part_2("1212"));
        assert_eq!(0, part_2("1221"));
        assert_eq!(4, part_2("123425"));
        assert_eq!(12, part_2("123123"));
        assert_eq!(4, part_2("12131415"));
    }
}
