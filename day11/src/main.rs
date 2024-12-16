use std::collections::HashMap;

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
    println!("puzzle2 = {}", puzzle2(str));
}

fn puzzle1(str: &str) -> usize {
    let nums = parse(str);
    nums.iter()
        .map(|&num| cycle(num, 25, &mut HashMap::new()))
        .sum()
}

fn puzzle2(str: &str) -> usize {
    let nums = parse(str);
    nums.iter()
        .map(|&num| cycle(num, 75, &mut HashMap::new()))
        .sum()
}

fn cycle(num: usize, count: i32, cache: &mut HashMap<(usize, i32), usize>) -> usize {
    if count == 0 {
        return 1;
    }
    if let Some(&n) = cache.get(&(num, count)) {
        return n;
    }
    let res = if num == 0 {
        cycle(1, count - 1, cache)
    } else if let Some(n) = even_digit(num) {
        let div = 10_usize.pow(n / 2);
        cycle(num / div, count - 1, cache) + cycle(num % div, count - 1, cache)
    } else {
        cycle(num * 2024, count - 1, cache)
    };
    cache.insert((num, count), res);
    res
}

fn even_digit(num: usize) -> Option<u32> {
    let count = digit_count(num);
    if count % 2 == 0 {
        Some(count)
    } else {
        None
    }
}

fn digit_count(mut num: usize) -> u32 {
    if num == 0 {
        return 1;
    }
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}

fn parse(str: &str) -> Vec<usize> {
    str.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "125 17";
        assert_eq!(puzzle1(input), 55312);
    }
}
