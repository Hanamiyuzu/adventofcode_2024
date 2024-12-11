use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
    println!("puzzle2 = {}", puzzle2(str));
}

fn puzzle1(str: &str) -> usize {
    let map = parse(str);
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    let mut antennas = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate().filter(|(_, &ch)| ch != '.') {
            antennas
                .entry(ch)
                .or_insert(Vec::new())
                .push((i as i32, j as i32));
        }
    }
    let mut ans = HashSet::new();
    for v in antennas.values() {
        for perm in v.iter().permutations(2) {
            let (dr, dc) = (perm[1].0 - perm[0].0, perm[1].1 - perm[0].1);
            let (row, col) = (perm[0].0 - dr, perm[0].1 - dc);
            if row >= 0 && row < rows && col >= 0 && col < cols {
                ans.insert((row, col));
            }
        }
    }
    ans.len()
}

fn puzzle2(str: &str) -> usize {
    let map = parse(str);
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    let mut antennas = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate().filter(|(_, &ch)| ch != '.') {
            antennas
                .entry(ch)
                .or_insert(Vec::new())
                .push((i as i32, j as i32));
        }
    }
    let mut ans = HashSet::new();
    for v in antennas.values() {
        for perm in v.iter().permutations(2) {
            let (dr, dc) = (perm[1].0 - perm[0].0, perm[1].1 - perm[0].1);
            let mut row = perm[0].0;
            let mut col = perm[0].1;
            loop {
                if row < 0 || row >= rows || col < 0 || col >= cols {
                    break;
                }
                ans.insert((row, col));
                row -= dr;
                col -= dc;
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

    #[test]
    fn test_puzzle2() {
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
        assert_eq!(puzzle2(input), 34);
    }
}
