use std::collections::HashSet;

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
}

fn puzzle1(str: &str) -> i32 {
    let map = parse(str);
    let mut sum = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate().filter(|(_, &c)| c == '0') {
            sum += dfs(&map, i as i32, j as i32, &mut HashSet::new());
        }
    }
    sum
}

fn dfs(map: &[Vec<char>], i: i32, j: i32, visited: &mut HashSet<(i32, i32)>) -> i32 {
    if i < 0 || i >= map.len() as i32 || j < 0 || j >= map[0].len() as i32 {
        return 0;
    }
    let c = map[i as usize][j as usize];
    match c {
        '.' => 0,
        '9' => visited.insert((i, j)) as i32,
        _ => [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
            .iter()
            .filter(|&&(i, j)| sub(safe_get(map, i, j), c) == 1)
            .map(|&(i, j)| dfs(map, i, j, visited))
            .sum(),
    }
}

fn safe_get(map: &[Vec<char>], i: i32, j: i32) -> char {
    if i < 0 || i >= map.len() as i32 || j < 0 || j >= map[0].len() as i32 {
        return '.';
    }
    map[i as usize][j as usize]
}

fn sub(a: char, b: char) -> i32 {
    a as i32 - b as i32
}

fn parse(str: &str) -> Vec<Vec<char>> {
    str.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input1 = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        assert_eq!(puzzle1(input1), 4);

        let input2 = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!(puzzle1(input2), 36);
    }
}
