fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
}

fn puzzle1(str: &str) -> usize {
    let str = parse(str);
    let total_count = str
        .chars()
        .step_by(2)
        .fold(0, |acc, x| acc + x as usize - '0' as usize);
    let mut sum = 0;
    let mut acc = 0;
    let mut dot_index = 0;
    for (i, x) in str.chars().enumerate() {
        let num = x as usize - '0' as usize;
        // if (number) else (dot)
        if i % 2 == 0 {
            for k in acc..(acc + num).min(total_count) {
                sum += k * (i / 2);
            }
        } else {
            for k in acc..(acc + num).min(total_count) {
                sum += k * get_prev_num(str, dot_index);
                dot_index += 1;
            }
        }
        acc += num;
        if acc >= total_count {
            break;
        }
    }
    sum
}

fn get_prev_num(mut str: &str, mut off: usize) -> usize {
    if str.len() % 2 == 0 {
        str = &str[..str.len() - 1];
    }
    for (i, x) in str.chars().rev().enumerate().step_by(2) {
        let num = x as usize - '0' as usize;
        if off == 0 || num > off {
            return (str.len() - 1 - i) / 2;
        } else {
            off -= num;
        }
    }
    0
}

fn parse(str: &str) -> &str {
    str
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle1() {
        assert_eq!(puzzle1("12345"), 60);
        assert_eq!(puzzle1("2333133121414131402"), 1928);
    }
}
