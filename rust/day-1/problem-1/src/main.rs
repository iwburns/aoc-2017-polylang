fn main() {
    let input = include_str!("./input.txt");
    println!("{}", get_total(input));
}

fn get_total(input: &str) -> u32 {
    let parsed = parse_to_ints(input);

    let iter_left = parsed.iter();
    let iter_right = parsed.iter().skip(1);

    let mut total = iter_left.zip(iter_right).fold(0, |acc, (left, right)| {
        if left == right {
            return acc + left;
        }
        acc
    });

    let first = parsed.iter().next().unwrap_or(&0);
    let last = parsed.iter().last().unwrap_or(&0);

    if first == last {
        total += first;
    }

    total
}

fn parse_to_ints(input: &str) -> Vec<u32> {
    input.chars().map(|item| parse_or_zero(item)).collect()
}

fn parse_or_zero(val: char) -> u32 {
    val.to_digit(10).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "1122";
        let total = get_total(input);
        assert_eq!(total, 3);
    }

    #[test]
    fn test2() {
        let input = "1111";
        let total = get_total(input);
        assert_eq!(total, 4);
    }

    #[test]
    fn test3() {
        let input = "1234";
        let total = get_total(input);
        assert_eq!(total, 0);
    }

    #[test]
    fn test4() {
        let input = "91212129";
        let total = get_total(input);
        assert_eq!(total, 9);
    }
}
