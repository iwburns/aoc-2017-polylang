#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct MemBlock {
    point: Point,
    value: u32,
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
    let memory = build_memory_space(input);
    println!("{:?}", memory.iter().last().unwrap().value);
}

fn build_memory_space(size: usize) -> Vec<MemBlock> {
    let mut memory = Vec::with_capacity(size as usize);

    memory.push(MemBlock { point: Point { x: 0, y: 0 }, value: 1 });

    let mut curr_move = Move::Right { total: 1, moves_left: 1 };

    for i in 1.. {
        let MemBlock { point: curr_point, .. } = memory[i - 1];

        let (next_move, point) = get_next_move_and_point(curr_move, curr_point);
        let value = get_adjacent_points_sum(point, &memory);

        memory.push(MemBlock { point, value });

        curr_move = next_move;

        if value as usize > size {
            break;
        }
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

fn get_adjacent_points_sum(p: Point, memory: &Vec<MemBlock>) -> u32 {
    let Point { x, y } = p;

    memory.iter().fold(0, |acc, block| {
        let Point { x: bx, y: by} = block.point;

        if (bx - x).abs() < 2 && (by - y).abs() < 2 {
            acc + block.value
        } else {
            acc
        }
    })
}


