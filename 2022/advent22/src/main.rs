const INPUT : &str = include_str!("../input.txt");
const DOWN : (isize, isize) = (1, 0);
const UP : (isize, isize) = (-1, 0);
const LEFT : (isize, isize) = (0, -1);
const RIGHT : (isize, isize) = (0, 1);

fn off_bounds(row : isize, col : isize, map : &Vec<&str>) -> bool {
    row < 0 || row >= map.len() as isize ||
        col < 0 || col >= map[row as usize].len() as isize ||
        map[row as usize].as_bytes()[col as usize] == b' '
}

fn step_offbounds(current_pos : (isize, isize), current_direction : (isize, isize)) -> ((isize, isize), (isize, isize)) {
    let (row, col) = current_pos;

    match (current_pos.0 / 50, current_pos.1 / 50) {
        (0, 1) => { // 1
            match current_direction {
                UP => ((3 * 50 + col%50, 0), RIGHT), // 9
                LEFT => ((2 * 50 + (49 - row), 0), RIGHT), // 6'
                _ => unreachable!(),
            }
        },
        (0, 2) => { // 2
            match current_direction {
                UP => ((4*50-1, col%50), UP), // 9
                DOWN => ((50 + col%50, 2*50-1), LEFT), // 4
                RIGHT => ((2*50 + (49 - row), 2*50-1), LEFT), // 7'
                _ => unreachable!(),
            }
        },
        (1, 1) => { // 4
            match current_direction {
                RIGHT => ((49, 2*50 + row%50), UP), // 2
                LEFT => ((2*50, row%50), DOWN), // 6
                _ => unreachable!(),
            }
        },
        (2, 0) => { // 6
            match current_direction {
                UP => ((50+col, 50), RIGHT), // 4
                LEFT => ((49-row%50, 50), RIGHT), // 1'
                _ => unreachable!(),
            }
        },
        (2, 1) => { // 7
            match current_direction {
                DOWN => ((col%50 + 3*50, 49), LEFT), // 9
                RIGHT => ((49-row%50, 3*50-1), LEFT), // 2'
                _ => unreachable!(),
            }
        },
        (3, 0) => { // 9
            match current_direction {
                DOWN => ((0, col + 2*50), DOWN), // 2
                LEFT => ((0, row%50 + 50), DOWN), // 1
                RIGHT => ((3*50-1, row%50 + 50), UP), // 7
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn next(current_pos : (isize, isize), current_direction : (isize, isize), map : &Vec<&str>) -> ((isize, isize), (isize, isize)) {
    let (mut row, mut col) = (current_pos.0 + current_direction.0, current_pos.1 + current_direction.1);
    let mut next_dir = current_direction;

    // If we went off bounds, go to other side of map, and keep going in direction until we hit the map again.
    //
    if off_bounds(row, col, map) {
        // Part 1:
        // match current_direction {
        //     (0, -1) => col = map[row as usize].len() as isize - 1,
        //     (0, 1) => col = 0,
        //     (1, 0) => row = 0,
        //     (-1, 0) => row = map.len() as isize - 1,
        //     _ => unreachable!(),
        // }

        // while off_bounds(row, col, map) {
        //     row += current_direction.0;
        //     col += current_direction.1;
        // }

        ((row, col), next_dir) = step_offbounds(current_pos, current_direction);

        if off_bounds(row, col, map) {
            println!("{:?}, {:?}, {row}, {col}", current_pos, current_direction);
            assert!(false);
        }
    }

    ((row, col), next_dir)
}

fn rotate(current_direction : (isize, isize), rotation : &str) -> (isize, isize) {
    // Rotation matrices:
    //
    // L:        R:
    // (0,  1)   (0, -1)
    // (-1, 0)   (1,  0)
    //
    match rotation {
        "L" => (current_direction.1 * -1, current_direction.0     ),
        "R" => (current_direction.1     , current_direction.0 * -1),
        _ => { println!("rotation: {rotation}"); unreachable!() },
    }
}

fn main() {
    let mut tokens = INPUT.split("\n\n");
    let map : Vec<&str> = tokens.next().unwrap().split('\n').collect();
    let text = tokens.next().unwrap();
    let mut directions = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(|c: char| c == 'R' || c == 'L') {
        if last != index {
            directions.push(&text[last..index]);
        }
        directions.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        directions.push(&text[last..]);
    }

    let mut current_pos : (isize, isize) = (0, 0);
    while map[current_pos.0 as usize].as_bytes()[current_pos.1 as usize] == b' ' {
        current_pos.1 += 1;
    }

    let mut current_direction : (isize, isize) = (0, 1);
    println!("{:?}, {:?}", current_pos, current_direction);
    for dir in directions {
        println!("dir: {}", dir.trim());
        if let Ok(steps) = dir.trim().parse::<usize>() {
            for _ in 0..steps {
                let (next_pos, next_dir) = next(current_pos, current_direction, &map);
                println!("{:?}, {:?}, {}", next_pos, next_dir, map[next_pos.0 as usize].as_bytes()[next_pos.1 as usize]);
                if map[next_pos.0 as usize].as_bytes()[next_pos.1 as usize] == b'#' {
                    break;
                }

                current_pos = next_pos;
                current_direction = next_dir;
            }
        }
        else {
            current_direction = rotate(current_direction, dir);
        }
        println!("{:?}, {:?}", current_pos, current_direction);
    }

    let password = (current_pos.0 + 1) * 1000 + 4 * (current_pos.1 + 1) + match current_direction {
        (0 ,  1) => 0,
        (0 , -1) => 2,
        (1 ,  0) => 1,
        (-1,  0) => 3,
        _ => unreachable!(),
    };

    println!("{:?}, {:?}, {password}", current_pos, current_direction);
}
