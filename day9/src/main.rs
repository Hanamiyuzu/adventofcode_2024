use std::cell::RefCell;

fn main() {
    let str = include_str!("../input.txt");
    println!("puzzle1 = {}", puzzle1(str));
    println!("puzzle2 = {}", puzzle2(str));
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

struct Elem {
    nums: Vec<usize>,
    dot_count: usize,
}

fn puzzle2(str: &str) -> usize {
    let str = parse(str);
    let vec = str
        .chars()
        .enumerate()
        .map(|(i, x)| {
            let count = x as usize - '0' as usize;
            if i % 2 == 0 {
                RefCell::new(Elem {
                    nums: vec![i / 2; count],
                    dot_count: 0,
                })
            } else {
                RefCell::new(Elem {
                    nums: vec![],
                    dot_count: count,
                })
            }
        })
        .collect::<Vec<_>>();
    for i in (0..vec.len()).rev() {
        let mut x = vec[i].borrow_mut();
        if !x.nums.is_empty() {
            for j in 0..i {
                let mut k = vec[j].borrow_mut();
                if k.dot_count >= x.nums.len() {
                    x.dot_count += x.nums.len();
                    k.dot_count -= x.nums.len();
                    k.nums.append(&mut x.nums);
                }
            }
        }
    }
    let mut sum = 0;
    let mut sum_index = 0;
    for x in vec.iter() {
        for &m in x.borrow().nums.iter() {
            sum += m * sum_index;
            sum_index += 1;
        }
        sum_index += x.borrow().dot_count;
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

    #[test]
    fn test_puzzle2() {
        assert_eq!(puzzle2("12345"), 132);
        assert_eq!(puzzle2("2333133121414131402"), 2858);
    }
}
