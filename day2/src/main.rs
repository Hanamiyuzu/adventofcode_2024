fn main() {
    let str = include_str!("../input.txt");
    puzzle1(str);
}

fn puzzle1(str: &str) -> i32 {
    let input = parse(str);
    let count = input
        .iter()
        .map(|v| {
            let d = v.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            match d[0].signum() {
                0 => false,
                1 => d.iter().all(|&x| x >= 1 && x <= 3),
                _ => d.iter().all(|&x| x >= -3 && x <= -1),
            }
        })
        .map(|x| x as i32)
        .sum();
    println!("puzzle1: {count}");
    count
}

fn parse(str: &str) -> Vec<Vec<i32>> {
    str.lines()
        .map(|x| {
            x.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(puzzle1(input), 2);
    }
}
