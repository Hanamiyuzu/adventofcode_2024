fn main() {
    let str = include_str!("../input.txt");
    puzzle1(str);
}

fn puzzle1(str: &str) -> i32 {
    0
}

fn parse(str: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        let input = "";
        assert_eq!(puzzle1(input), 11);
    }
}
