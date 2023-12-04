use std::collections::HashSet;
use std::cmp::max;
const INPUT : &str = include_str!("../input2.txt");

type Coord = (isize, isize);

fn main() {
    let moves = INPUT.trim().as_bytes();

    let rocks = [
        vec![(0,0), (0,1), (0,2), (0,3)],
        vec![(0,1), (1,0), (1,1), (1,2), (2,1)],
        vec![(0,0), (0,1), (0,2), (1,2), (2,2)],
        vec![(0,0), (1,0), (2,0), (3,0)],
        vec![(0,0), (0,1), (1,0), (1,1)],
    ];

    // part 1
    println!("part 1: {}", part1(moves, &rocks, 2022));

    // part 2 - keep going until we find a cycle.
    let mut occupied : HashSet<Coord> = HashSet::new();

    let mut current_move = 0;
    let mut current_rock = 0;
    let mut current_height = 0;

    let mut previous_floor_move = 0;
    let mut previous_floor_rock = 0;

    let mut first_floor_of_cycle = 0;
    let mut height_at_first_floor = 0;
    let mut cycle_height = 0;
    let mut cycle_period = 0;

    for i in 0.. {
        let mut rock : Vec<Coord> = rocks[current_rock].iter()
            .map(|(r, c)| (*r + current_height + 3, *c + 2)).collect();
        move_rock(moves, &mut current_move, &mut rock, &occupied);
        current_height = max(current_height, rock.iter().map(|(r, _)| *r).max().unwrap() + 1);

        for p in rock {
            occupied.insert(p);
        }

        current_rock = (current_rock + 1) % rocks.len();
        // render(current_height, &occupied);

        // Did we make a new floor?
        let mut floor = true;
        for c in 0..7 {
            if !occupied.contains(&(current_height-1, c)) {
                floor = false;
                break;
            }
        }

        if floor {
            println!("Found new floor after {i} rocks, height: {current_height}");
            if previous_floor_move == current_move && previous_floor_rock == current_rock {
                cycle_period = i - first_floor_of_cycle;
                cycle_height = current_height - height_at_first_floor;
                println!("Success! We found a cycle:");
                println!(" that repeats every {cycle_period} rocks");
                println!(" and increases the height by {cycle_height} on every iteration");
                println!(" and starts at floor {first_floor_of_cycle}.");
                break;
            } else {
                first_floor_of_cycle = i;
                height_at_first_floor = current_height;
                previous_floor_move = current_move;
                previous_floor_rock = current_rock;
            }
        }
    }

    let rock_count = 1000000000000;
    let modrocks = (rock_count - first_floor_of_cycle) % (cycle_period);
    let modheight = part1(moves, &rocks, first_floor_of_cycle + modrocks) - height_at_first_floor;
    let total_height = height_at_first_floor + ((rock_count - first_floor_of_cycle) / cycle_period) * cycle_height + modheight;
    println!("{total_height}");
}

fn part1(moves: &[u8], rocks: &[Vec<Coord>], rock_count : isize) -> isize {
    let mut occupied : HashSet<Coord> = HashSet::new();

    let mut current_move = 0;
    let mut current_rock = 0;
    let mut current_height = 0;

    for _ in 0..rock_count {
        let mut rock : Vec<Coord> = rocks[current_rock].iter()
            .map(|(r, c)| (*r + current_height + 3, *c + 2)).collect();
        move_rock(moves, &mut current_move, &mut rock, &occupied);
        current_height = max(current_height, rock.iter().map(|(r, _)| *r).max().unwrap() + 1);

        for p in rock {
            occupied.insert(p);
        }

        current_rock = (current_rock + 1) % rocks.len();
    }

    current_height
}

fn jet_rock(rock : &Vec<Coord>, m : u8) -> Vec<Coord> {
    match m {
        b'>' => rock.iter().cloned().map(|(r, c)| (r, c+1)).collect(),
        b'<' => rock.iter().cloned().map(|(r, c)| (r, c-1)).collect(),
        _ => unreachable!(),
    }
}

fn drop_rock(rock : &Vec<Coord>) -> Vec<Coord> {
    rock.iter().cloned().map(|(r, c)| (r-1, c)).collect()
}

fn validate_rock(rock : &Vec<Coord>, occupied : &HashSet<Coord>) -> bool {
    for (r, c) in rock.iter().cloned() {
        if occupied.contains(&(r, c)) || c < 0 || c > 6 || r < 0 {
            return false;
        }
    }

    true
}

fn move_rock(moves : &[u8], current_move : &mut usize, rock : &mut Vec<Coord>, occupied : &HashSet<Coord>) {
    loop {
        let next_rock = jet_rock(rock, moves[*current_move]);
        *current_move = (*current_move+1) % moves.len();

        if validate_rock(&next_rock, occupied) {
            *rock = next_rock;
        }

        let next_rock = drop_rock(rock);
        if validate_rock(&next_rock, occupied) {
            *rock = next_rock;
        } else {
            break;
        }
    }
}

fn render(current_height : isize, occupied : &HashSet<Coord>) {
    for r in ((current_height-20)..current_height).rev() {
        let row : String = (0..7).map(|c| if occupied.contains(&(r, c)) { '#' } else { '.' }).collect();
        println!("{row}");
    }
    println!("");
}
