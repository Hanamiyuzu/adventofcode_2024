use regex::Regex;

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
    println!("puzzle2 = {}", puzzle2(str));
}

fn puzzle1(str: &str) -> i32 {
    let str = parse(str);
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(str)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum()
}

fn puzzle2(str: &str) -> i32 {
    let mut str = parse(str);
    let mut sum = 0;
    while let Some(pos) = str.find("don't()") {
        sum += puzzle1(&str[..pos]);
        str = str[pos..].find("do()").map_or("", |pp| &str[pos + pp..]);
    }
    sum += puzzle1(str);
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

    #[test]
    fn test_puzzle2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(puzzle2(input), 48);
    }
}
