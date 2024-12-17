use std::collections::HashSet;

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
}

fn puzzle1(str: &str) -> usize {
    let map = parse(str);
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for (row, s) in map.iter().enumerate() {
        for (col, _) in s.iter().enumerate() {
            let (row, col) = (row as i32, col as i32);
            if !visited.contains(&(row, col)) {
                regions.push(collect_cells(&map, row, col, &mut visited));
            }
        }
    }
    regions
        .iter()
        .map(|re| re.iter().map(|(.., edge)| edge).sum::<usize>() * re.len())
        .sum()
}

fn collect_cells(
    map: &[Vec<char>],
    row: i32,
    col: i32,
    visited: &mut HashSet<(i32, i32)>,
) -> Vec<(i32, i32, usize)> {
    let elem = map[row as usize][col as usize];
    let mut res = vec![];
    let mut to_visit = vec![(row, col)];
    while let Some((row, col)) = to_visit.pop() {
        if !visited.insert((row, col)) {
            continue;
        }
        res.push((row, col, 0));
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nrow, ncol) = (row + dx, col + dy);
            if safe_cell(map, nrow, ncol) && map[nrow as usize][ncol as usize] == elem {
                to_visit.push((nrow, ncol));
            } else {
                res.last_mut().unwrap().2 += 1; // edge count
            }
        }
    }
    res
}

fn safe_cell(map: &[Vec<char>], row: i32, col: i32) -> bool {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    row >= 0 && row < rows && col >= 0 && col < cols
}

fn parse(str: &str) -> Vec<Vec<char>> {
    str.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let str = "AAAA
BBCD
BBCC
EEEC";
        assert_eq!(puzzle1(str), 140);

        let str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        assert_eq!(puzzle1(str), 772);

        let str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(puzzle1(str), 1930);
    }
}
