fn main() {
    let input = include_str!("./input.txt");
    println!("{}", compute_checksum(input));
}

fn compute_checksum(input: &str) -> u32 {
    input.lines().map(|line| get_line_div(line)).sum()
}

fn get_line_div(line: &str) -> u32 {
    let numbers: Vec<u32> = line
        .split_whitespace()
        .map(|num| num.parse().unwrap_or(0))
        .collect();

    for (index, a) in numbers.iter().enumerate() {
        for b in numbers.iter().skip(index + 1) {
            if a % b == 0 {
                return a / b;
            } else if b % a == 0 {
                return b / a;
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input =
              "5 9 2 8\
             \n9 4 7 3\
             \n3 8 6 5";

        assert_eq!(compute_checksum(input), 9);
    }
}
