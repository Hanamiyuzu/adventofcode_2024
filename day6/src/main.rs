use std::collections::HashSet;

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
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
            dir.turn_right();
        } else {
            (row, col) = (nrow, ncol);
        }
    }
}

#[derive(Debug)]
enum DIR {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl DIR {
    fn turn_right(&mut self) {
        *self = match self {
            DIR::UP => DIR::RIGHT,
            DIR::RIGHT => DIR::DOWN,
            DIR::DOWN => DIR::LEFT,
            DIR::LEFT => DIR::UP,
        };
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
}
