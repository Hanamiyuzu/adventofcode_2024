use regex::Regex;

fn main() {
    let str = include_str!("../input.txt");
    puzzle1(str);
}

fn puzzle1(str: &str) -> i32 {
    let str = parse(str);
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = re
        .captures_iter(str)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum();
    println!("puzzle1 = {}", sum);
    sum
}

fn parse(str: &str) -> &str {
    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(puzzle1(input), 161);
    }
}
