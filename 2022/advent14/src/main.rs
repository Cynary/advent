use std::collections::HashSet;
use std::cmp::{max, min};

const INPUT : &str = include_str!("../input.txt");

fn main() {
    let mut map = HashSet::<(isize, isize)>::new();
    for line in INPUT.trim().split('\n') {
        let coordinates : Vec<Vec<isize>> = line.trim()
            .split("->")
            .map(
                |c| c.trim()
                    .split(',')
                    .map(|n| n.parse::<isize>().unwrap()).collect())
            .collect();

        for i in 0..(coordinates.len()-1) {
            let x_start = min(coordinates[i][0], coordinates[i+1][0]);
            let y_start = min(coordinates[i][1], coordinates[i+1][1]);

            let x_end = max(coordinates[i][0], coordinates[i+1][0]);
            let y_end = max(coordinates[i][1], coordinates[i+1][1]);

            assert!(x_start == x_end || y_start == y_start);
            if x_start == x_end {
                for y in y_start..(y_end+1) {
                    map.insert((x_start, y));
                }
            } else {
                for x in x_start..(x_end+1) {
                    map.insert((x, y_start));
                }
            }
        }
    }

    let max_y = *map.iter().map(|(_,y)| y).max().unwrap();

    let mut sands1 = -1;
    let mut sands = 0;
    loop {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y == max_y + 1 {
                break;
            }

            if ! map.contains(&(x, y+1)) {
                y += 1;
            } else if ! map.contains(&(x-1, y+1)) {
                x -= 1;
                y += 1;
            } else if ! map.contains(&(x+1, y+1)) {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        if y >= max_y && sands1 == -1 {
            sands1 = sands;
        }

        map.insert((x, y));
        sands += 1;

        if (x, y) == (500, 0) {
            break;
        }
    }

    println!("{max_y}, {sands1}, {sands}");
}
