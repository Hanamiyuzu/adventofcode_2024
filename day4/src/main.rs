fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
    println!("puzzle2 = {}", puzzle2(str));
}

fn puzzle1(str: &str) -> i32 {
    let map = parse(str);
    let mut count = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate().filter(|(_, &ch)| ch == 'X') {
            count += DIRS
                .iter()
                .map(|dir| search(&map, i as i32, j as i32, ch, dir))
                .sum::<i32>();
        }
    }
    count
}

fn puzzle2(str: &str) -> i32 {
    let map = parse(str);
    let mut count = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate().filter(|(_, &ch)| ch == 'A') {
            let a = safe_get(&map, i as i32 - 1, j as i32 - 1);
            let b = safe_get(&map, i as i32 + 1, j as i32 + 1);
            let c = safe_get(&map, i as i32 + 1, j as i32 - 1);
            let d = safe_get(&map, i as i32 - 1, j as i32 + 1);
            count += (((a == 'M' && b == 'S') || (a == 'S' && b == 'M'))
                && ((c == 'M' && d == 'S') || (c == 'S' && d == 'M'))) as i32;
        }
    }
    count
}

#[rustfmt::skip]
const DIRS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)
];

fn search(map: &Vec<Vec<char>>, i: i32, j: i32, ch: char, dir: &(i32, i32)) -> i32 {
    if safe_get(map, i, j) != ch {
        return 0;
    }
    match ch {
        'S' => 1,
        'X' => search(map, i + dir.0, j + dir.1, 'M', dir),
        'M' => search(map, i + dir.0, j + dir.1, 'A', dir),
        'A' => search(map, i + dir.0, j + dir.1, 'S', dir),
        _ => 0,
    }
}

fn safe_get(map: &Vec<Vec<char>>, i: i32, j: i32) -> char {
    if i < 0 || i >= map.len() as i32 || j < 0 || j >= map[0].len() as i32 {
        return '.';
    }
    map[i as usize][j as usize]
}

fn parse(str: &str) -> Vec<Vec<char>> {
    str.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(puzzle1(input), 18);
    }

    #[test]
    fn test_puzzle2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(puzzle2(input), 9);
    }
}
