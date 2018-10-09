#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct MemBlock {
    point: Point,
    address: usize,
}

#[derive(Debug, Copy, Clone)]
enum Move {
    Right { total: u32, moves_left: u32 },
    Left { total: u32, moves_left: u32 },
    Up { total: u32, moves_left: u32 },
    Down { total: u32, moves_left: u32 },
}

fn main() {
    let input = 347991;
    let dist = find_distance_to_mid(input);
    println!("{:?}", dist);
}

fn build_memory_space(size: usize) -> Vec<MemBlock> {
    let mut memory = Vec::with_capacity(size as usize);

    memory.push(MemBlock { point: Point { x: 0, y: 0 }, address: 1 });

    let mut curr_move = Move::Right { total: 1, moves_left: 1 };

    for i in 1..size {
        let MemBlock { point: curr_point, .. } = memory[i - 1];

        let (next_move, next_point) = get_next_move_and_point(curr_move, curr_point);

        memory.push(MemBlock { point: next_point, address: i + 1 });

        curr_move = next_move;
    }

    memory
}

fn get_next_move_and_point(curr_move: Move, curr_point: Point) -> (Move, Point) {
    let Point { x, y } = curr_point;

    match curr_move {
        Move::Right { total, moves_left } => {
            let next_point = Point { x: x + 1, y };

            let next_move = if moves_left > 1 {
                Move::Right { total, moves_left: moves_left - 1}
            } else {
                Move::Up { total, moves_left: total }
            };

            (next_move, next_point)
        },
        Move::Left { total, moves_left } => {
            let next_point = Point { x: x - 1, y };

            let next_move = if moves_left > 1 {
                Move::Left { total, moves_left: moves_left - 1}
            } else {
                Move::Down { total, moves_left: total }
            };

            (next_move, next_point)
        },
        Move::Up { total, moves_left } => {
            let next_point = Point { x, y: y + 1 };

            let next_move = if moves_left > 1 {
                Move::Up { total, moves_left: moves_left - 1}
            } else {
                Move::Left { total: total + 1, moves_left: total + 1 }
            };

            (next_move, next_point)
        },
        Move::Down { total, moves_left } => {
            let next_point = Point { x, y: y - 1 };

            let next_move = if moves_left > 1 {
                Move::Down { total, moves_left: moves_left - 1}
            } else {
                Move::Right { total: total + 1, moves_left: total + 1 }
            };

            (next_move, next_point)
        },
    }
}

fn find_distance_to_mid(input: usize) -> i32 {
    let memory = build_memory_space(input);
    let point = memory[memory.len() - 1].point;
    point.x.abs() + point.y.abs()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let input = 1;
        let dist = find_distance_to_mid(input);
        assert_eq!(dist, 0);
    }

    #[test]
    fn test2() {
        let input = 12;
        let dist = find_distance_to_mid(input);
        assert_eq!(dist, 3);
    }

    #[test]
    fn test3() {
        let input = 23;
        let dist = find_distance_to_mid(input);
        assert_eq!(dist, 2);
    }

    #[test]
    fn test4() {
        let input = 1024;
        let dist = find_distance_to_mid(input);
        assert_eq!(dist, 31);
    }
}

