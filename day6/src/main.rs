use std::collections::HashSet;

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
    println!("puzzle2 = {}", puzzle2(str));
}

fn puzzle1(str: &str) -> usize {
    let (map, (mut row, mut col, mut dir)) = parse(str);
    let mut visited = HashSet::new();
    loop {
        visited.insert((row, col));
        let (nrow, ncol) = dir.walk(row, col);
        if nrow < 0 || nrow >= map.len() as i32 || ncol < 0 || ncol >= map[0].len() as i32 {
            return visited.len();
        }
        if map[nrow as usize][ncol as usize] == '#' {
            dir = dir.turn_right();
        } else {
            (row, col) = (nrow, ncol);
        }
    }
}

// If a right turn at a certain location would make the guard to retrace its previous path, then that location could be considered a blocking point.
// However, some valid blocking points might never be encountered during regular movement, potentially leading to missing information.
// I'm trying to figure out how to account for these missing cases.
fn puzzle2_fail(str: &str) -> usize {
    let (map, (mut row, mut col, mut dir)) = parse(str);
    let mut row_path = HashSet::new();
    let mut col_path = HashSet::new();
    let mut obstruction = HashSet::new();
    loop {
        row_path.insert((row, dir));
        col_path.insert((col, dir));
        let (nrow, ncol) = dir.walk(row, col);
        if nrow < 0 || nrow >= map.len() as i32 || ncol < 0 || ncol >= map[0].len() as i32 {
            //println!("{:?}", obstruction);
            return obstruction.len();
        }
        if map[nrow as usize][ncol as usize] == '#' {
            dir = dir.turn_right();
        } else {
            match dir {
                DIR::UP | DIR::DOWN => {
                    if row_path.contains(&(row, dir.turn_right())) {
                        obstruction.insert((nrow, ncol));
                    }
                }
                DIR::LEFT | DIR::RIGHT => {
                    if col_path.contains(&(col, dir.turn_right())) {
                        obstruction.insert((nrow, ncol));
                    }
                }
            }
            (row, col) = (nrow, ncol);
        }
    }
}

fn puzzle2(str: &str) -> usize {
    let (map, start) = parse(str);
    let (mut row, mut col, mut dir) = start;
    let mut obstruction = HashSet::new();
    loop {
        let (nrow, ncol) = dir.walk(row, col);
        if nrow < 0 || nrow >= map.len() as i32 || ncol < 0 || ncol >= map[0].len() as i32 {
            return obstruction.len();
        }
        if map[nrow as usize][ncol as usize] == '#' {
            dir = dir.turn_right();
        } else {
            let mut temp = map.clone();
            temp[nrow as usize][ncol as usize] = '#';
            if !obstruction.contains(&(nrow, ncol)) && is_inf_loop(&temp, start) {
                obstruction.insert((nrow, ncol));
            }
            (row, col) = (nrow, ncol);
        }
    }
}

fn is_inf_loop(map: &Vec<Vec<char>>, (mut row, mut col, mut dir): (i32, i32, DIR)) -> bool {
    let mut visited = HashSet::new();
    loop {
        if visited.contains(&(row, col, dir)) {
            return true;
        }
        visited.insert((row, col, dir));
        let (nrow, ncol) = dir.walk(row, col);
        if nrow < 0 || nrow >= map.len() as i32 || ncol < 0 || ncol >= map[0].len() as i32 {
            return false;
        }
        if map[nrow as usize][ncol as usize] == '#' {
            dir = dir.turn_right();
        } else {
            (row, col) = (nrow, ncol);
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum DIR {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl DIR {
    fn turn_right(&self) -> Self {
        match self {
            DIR::UP => DIR::RIGHT,
            DIR::RIGHT => DIR::DOWN,
            DIR::DOWN => DIR::LEFT,
            DIR::LEFT => DIR::UP,
        }
    }

    fn walk(&self, row: i32, col: i32) -> (i32, i32) {
        match self {
            DIR::UP => (row - 1, col),
            DIR::DOWN => (row + 1, col),
            DIR::LEFT => (row, col - 1),
            DIR::RIGHT => (row, col + 1),
        }
    }
}

fn parse(str: &str) -> (Vec<Vec<char>>, (i32, i32, DIR)) {
    let mut start = (0, 0, DIR::UP);
    let map = str
        .lines()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == '^' {
                        start = (row as i32, col as i32, DIR::UP);
                    }
                    c
                })
                .collect()
        })
        .collect();
    (map, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(puzzle1(input), 41);
    }

    #[test]
    fn test_puzzle2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(puzzle2(input), 6);
    }
}
