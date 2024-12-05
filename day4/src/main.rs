fn main() {
    let str = include_str!("../input.txt");
    puzzle1(str);
}

fn puzzle1(str: &str) -> i32 {
    let map = parse(str);
    let count = map
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &ch)| ch == 'X')
                .map(|(j, &ch)| {
                    DIRS.iter()
                        .map(|dir| search(&map, i as i32, j as i32, ch, dir))
                        .sum::<i32>()
                })
                .sum::<i32>()
        })
        .sum();
    println!("puzzle1 = {}", count);
    count
}

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn search(map: &Vec<Vec<char>>, i: i32, j: i32, ch: char, dir: &(i32, i32)) -> i32 {
    if i < 0 || i >= map.len() as i32 || j < 0 || j >= map[0].len() as i32 {
        return 0;
    }
    if map[i as usize][j as usize] != ch {
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
}
