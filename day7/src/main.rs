fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
}

fn puzzle1(str: &str) -> i64 {
    let inputs = parse(str);
    inputs
        .iter()
        .filter(|(test, nums)| calc(*test, &nums[1..], nums[0]))
        .map(|(x, _)| x)
        .sum()
}

fn calc(test: i64, nums: &[i64], sum: i64) -> bool {
    if nums.is_empty() {
        test == sum
    } else {
        calc(test, &nums[1..], sum + nums[0]) || calc(test, &nums[1..], sum * nums[0])
    }
}

fn parse(str: &str) -> Vec<(i64, Vec<i64>)> {
    str.lines()
        .map(|s| {
            let (test, numbers) = s.split_once(':').unwrap();
            (
                test.parse::<i64>().unwrap(),
                numbers
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(puzzle1(input), 3749);
    }
}
