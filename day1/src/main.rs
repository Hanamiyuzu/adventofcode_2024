fn main() {
    let str = include_str!("../input.txt");
    puzzle1(str);
}

fn puzzle1(str: &str) -> i32 {
    let (mut s1, mut s2) = parse(str);
    s1.sort_unstable();
    s2.sort_unstable();
    let sum = s1
        .iter()
        .zip(s2.iter())
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum::<i32>();
    println!("puzzle1 = {sum}");
    sum
}

fn parse(str: &str) -> (Vec<i32>, Vec<i32>) {
    str.lines()
        .map(|x| x.split_whitespace().take(2).collect::<Vec<_>>())
        .map(|vec| {
            (
                vec[0].parse::<i32>().unwrap(),
                vec[1].parse::<i32>().unwrap(),
            )
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(puzzle1(input), 11);
    }
}
