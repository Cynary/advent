use std::collections::VecDeque;
use std::collections::HashSet;
use std::cmp::min;

const INPUT : &str = include_str!("../input.txt");
const GRID : &[u8] = INPUT.as_bytes();

struct Grid {
    rows : usize,
    columns : usize,
    start : (usize, usize),
    end : (usize, usize),

    starts : Vec<(usize, usize)>,
}

impl Grid {
    pub fn new() -> Self {
        let columns = INPUT.find('\n').unwrap();
        let rows = INPUT.len() / columns;

        let mut start : (usize, usize) = (0,0);
        let mut end : (usize, usize) = (0,0);
        let mut starts = Vec::<(usize, usize)>::new();

        for r in 0..rows {
            for c in 0..columns {
                let spot = GRID[(r * (columns + 1) + c) as usize];
                if spot == b'E' {
                    end = (r, c);
                } else if spot == b'S' {
                    start = (r, c);
                    starts.push((r, c));
                }
                else if spot == b'a' {
                    starts.push((r, c));
                }
            }
        }

        Self { rows : rows, columns : columns, start : start, end : end, starts: starts }
    }

    fn at(&self, r : isize, c : isize) -> u8 {
        if r < 0 || c < 0 || r >= self.rows as isize || c >= self.columns as isize {
            b'z' - b'a' + 10
        } else {
            match GRID[(r * (self.columns as isize+1) + c) as usize] {
                b'S' => 0,
                b'E' => b'z'-b'a',
                x => x-b'a',
            }
        }
    }

    fn neighbors(&self, r : usize, c : usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::<(usize, usize)>::new();
        let cheight = self.at(r as isize, c as isize);
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nheight = self.at(r as isize + dr, c as isize + dc);
            if nheight <= cheight+1 {
                assert!(r < self.rows && c < self.columns);
                neighbors.push(((r as isize+dr) as usize, (c as isize+dc) as usize))
            }
        }

        neighbors
    }
}

fn shortest(grid : &Grid, start : (usize, usize)) -> usize {
    let mut q = VecDeque::<(usize, (usize, usize))>::new();
    let mut visited = HashSet::<(usize, usize)>::new();
    let mut min_dist : usize = 10000000000;

    q.push_back((0, start));
    visited.insert(start);
    while q.len() != 0 {
        let (distance, (r, c)) = q.pop_front().unwrap();
        if (r,c) == grid.end {
            min_dist = distance;
            break;
        }

        let neighbors = grid.neighbors(r, c);
        for n in neighbors {
            if ! visited.contains(&n) {
                visited.insert(n);
                q.push_back((distance+1,n));
            }
        }
    }

    min_dist
}

fn main() {
    let grid = Grid::new();
    println!("{}", shortest(&grid, grid.start));
    println!("{}", grid.starts.iter().map(|s| shortest(&grid, *s)).min().unwrap());
}
