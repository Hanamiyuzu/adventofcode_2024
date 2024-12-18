use std::collections::HashMap;

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
    println!("puzzle2 = {}", puzzle2(str));
}

fn puzzle1(str: &str) -> i32 {
    let (mut s1, mut s2) = parse(str);
    s1.sort_unstable();
    s2.sort_unstable();
    s1.iter()
        .zip(s2.iter())
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum::<i32>()
}

fn puzzle2(str: &str) -> i32 {
    let (s1, s2) = parse(str);
    let mut count_map = HashMap::new();
    for &x in s2.iter() {
        *count_map.entry(x).or_insert(0) += 1;
    }
    s1.iter()
        .map(|x| count_map.get(x).map_or(0, |c| x * c))
        .sum()
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

    #[test]
    fn test_puzzle2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(puzzle2(input), 31);
    }
}
