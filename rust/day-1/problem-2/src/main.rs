fn main() {
    let input = include_str!("./input.txt");
    println!("{}", get_total(input));
}

fn get_total(input: &str) -> u32 {
    let parsed = parse_to_ints(input);

    let compare_distance = parsed.len() / 2;

    let iter_left = parsed.iter();
    let iter_right = parsed.iter().cycle().skip(compare_distance);

    iter_left.zip(iter_right).fold(0, |acc, (left, right)| {
        if left == right {
            return acc + left;
        }
        acc
    })
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
        let input = "1212";
        let total = get_total(input);
        assert_eq!(total, 6);
    }

    #[test]
    fn test2() {
        let input = "1221";
        let total = get_total(input);
        assert_eq!(total, 0);
    }

    #[test]
    fn test3() {
        let input = "123425";
        let total = get_total(input);
        assert_eq!(total, 4);
    }

    #[test]
    fn test4() {
        let input = "123123";
        let total = get_total(input);
        assert_eq!(total, 12);
    }

    #[test]
    fn test5() {
        let input = "12131415";
        let total = get_total(input);
        assert_eq!(total, 4);
    }
}
