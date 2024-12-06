fn main() {
    let str = include_str!("../input.txt");
    puzzle1(str);
    puzzle2(str);
}

fn puzzle1(str: &str) -> usize {
    let (rules, input) = parse(str);
    let sum = input
        .iter()
        .map(|v| {
            if is_match_rules(&rules, v) {
                v[v.len() / 2]
            } else {
                0
            }
        })
        .sum();
    println!("puzzle1 = {}", sum);
    sum
}

fn puzzle2(str: &str) -> usize {
    let (rules, input) = parse(str);
    let sum = input
        .iter()
        .map(|v| {
            if !is_match_rules(&rules, v) {
                let mut vv = v.clone();
                vv.sort_unstable_by(|&a, &b| {
                    if rules[b][a] {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                });
                return vv[vv.len() / 2];
            }
            0
        })
        .sum();
    println!("puzzle2 = {}", sum);
    sum
}

fn is_match_rules(rules: &[[bool; 100]; 100], v: &[usize]) -> bool {
    v.iter()
        .enumerate()
        .all(|(i, &x)| v.iter().skip(i + 1).all(|&y| !rules[y][x]))
}

fn parse(str: &str) -> ([[bool; 100]; 100], Vec<Vec<usize>>) {
    let (s1, s2) = str.split_once("\n\n").unwrap();
    let mut rules = [[false; 100]; 100];
    s1.lines().for_each(|s| {
        let (a, b) = s.split_once('|').unwrap();
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();
        rules[a][b] = true;
    });
    let input = s2
        .lines()
        .map(|s| s.split(',').map(|x| x.parse::<usize>().unwrap()).collect())
        .collect();
    (rules, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(puzzle1(input), 143);
    }

    #[test]
    fn test_puzzle2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(puzzle2(input), 123);
    }
}
