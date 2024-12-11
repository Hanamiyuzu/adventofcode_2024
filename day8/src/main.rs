use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
}

fn puzzle1(str: &str) -> usize {
    let map = parse(str);
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    let mut antennas = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate().filter(|(_, &ch)| ch != '.') {
            let (i, j) = (i as i32, j as i32);
            antennas.entry(ch).or_insert(Vec::new()).push((i, j));
        }
    }
    let mut ans = HashSet::new();
    for v in antennas.values() {
        for perm in v.iter().permutations(2) {
            let (dx, dy) = (perm[1].0 - perm[0].0, perm[1].1 - perm[0].1);
            let (x, y) = (perm[0].0 - dx, perm[0].1 - dy);
            if x >= 0 && x < cols && y >= 0 && y < rows {
                ans.insert((x, y));
            }
        }
    }
    ans.len()
}

fn parse(str: &str) -> Vec<Vec<char>> {
    str.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!(puzzle1(input), 14);
    }
}
