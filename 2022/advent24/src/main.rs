use std::collections::{HashSet,BinaryHeap};

const INPUT : &str = include_str!("../input.txt");

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Copy, Clone)]
struct Position {
    row : i16,
    col : i16,
}

impl Position {
    pub fn new(row : i16, col : i16) -> Self {
        Self {row, col}
    }

    fn neighbors(&self) -> Vec<Self> {
        return vec![
            Self::new(self.row+1, self.col),
            Self::new(self.row-1, self.col),
            Self::new(self.row, self.col+1),
            Self::new(self.row, self.col-1),
            Self::new(self.row, self.col),
        ]
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Blizzard {
    pos : Position,
    dir : u8,
}

impl Blizzard {
    pub fn new(row : i16, col : i16, dir : &str) -> Self {
        Self { pos : Position::new(row, col), dir : dir.as_bytes()[0] }
    }
}

struct BlizzardOccVec {
    occ : Vec<u64>,
    max_row : i16,
    max_col : i16,
}

impl BlizzardOccVec {
    pub fn new(max_row : i16, max_col : i16) -> Self {
        Self { occ : vec![0; ((((max_row+1) * (max_col+1))/64)+1) as usize], max_row, max_col }
    }

    fn new_occ(&mut self) {
        self.occ.resize(self.occ.len() + ((((self.max_row+1) * (self.max_col+1))/64)+1) as usize, 0);
    }

    fn set(&mut self, i : usize, r : i16, c : i16) {
        self.occ[i*((((self.max_row+1) * (self.max_col+1))/64)+1) as usize +
                 ((r*self.max_col + c)/64) as usize] |= 1 << ((r*self.max_col + c)%64);
    }

    fn is_set(&self, i : usize, r : i16, c : i16) -> bool {
        0 != self.occ[i*((((self.max_row+1) * (self.max_col+1))/64)+1) as usize +
                     ((r*self.max_col + c)/64) as usize] & (1 << ((r*self.max_col + c)%64))
    }

    fn len(&self) -> usize {
        self.occ.len() / ((((self.max_row+1) * (self.max_col+1))/64)+1) as usize
    }
}

fn main() {
    let mut start_position = Position::new(0,0);
    let mut goal_position = Position::new(0,0);
    let mut blizzard : Vec<Blizzard> = Vec::new();

    let max_row : i16 = INPUT.lines().count() as i16 - 1;
    let mut max_col : i16 = 0;

    for (row, line) in INPUT.lines().enumerate() {
        if row == 0 {
            start_position.col = line.match_indices('.').nth(0).unwrap().0 as i16;
            max_col = line.len() as i16 - 1;
        }

        if row == INPUT.lines().count()-1 {
            goal_position.row = row as i16;
            goal_position.col = line.match_indices('.').nth(0).unwrap().0 as i16;
        }

        for (col, dir) in line.match_indices(['>', 'v', '<', '^']) {
            blizzard.push(Blizzard::new(row as i16, col as i16, dir));
        }
    }

    let mut blizzards_occ = BlizzardOccVec::new(max_row, max_col);
    for blizz in &blizzard {
        blizzards_occ.set(0, blizz.pos.row, blizz.pos.col);
    }

    let goal_step0 = find_path(start_position, goal_position, &mut blizzard, &mut blizzards_occ, max_row, max_col, 0);
    let start_step = find_path(goal_position, start_position, &mut blizzard, &mut blizzards_occ, max_row, max_col, goal_step0);
    let goal_step1 = find_path(start_position, goal_position, &mut blizzard, &mut blizzards_occ, max_row, max_col, start_step);
    println!(
        "{:?}, {:?}, {}, ({goal_step0},{start_step},{goal_step1})",
        start_position,
        goal_position,
        blizzard.len());
}

fn find_path(
    start_pos : Position,
    goal_pos : Position, blizzard : &mut Vec<Blizzard>,
    blizzards_occ : &mut BlizzardOccVec,
    max_row : i16,
    max_col : i16,
    initial_step : i16) -> i16
{
    let mut q : BinaryHeap<(i16, i16, Position)> = BinaryHeap::new(); // (step+heuristic, step, current_position, trip_counter)
    let mut visited : HashSet<(i16, Position)> = HashSet::new(); // (step, current_position)
    visited.insert((initial_step, start_pos));
    q.push((-heuristic(start_pos, goal_pos), initial_step, start_pos));

    while let Some((_, step, pos)) = q.pop() {
        if pos == goal_pos {
            return step;
        }

        if blizzards_occ.len() <= (step+1) as usize {
            compute_next_blizzards_occ(blizzards_occ, blizzard, max_row, max_col);
        }

        for n in pos.neighbors() {
            if visited.contains(&(step+1, n)) || (n != start_pos && n != goal_pos && (n.row <= 0 || n.row >= max_row || n.col <= 0 || n.col >= max_col)) {
                continue;
            }

            if blizzards_occ.is_set((step+1) as usize, n.row, n.col)
            {
                continue;
            }

            visited.insert((step+1, n));
            q.push((-(step+1+heuristic(n, goal_pos)), step+1, n));
        }
    }

    unreachable!();
}

fn heuristic(pos : Position, goal : Position) -> i16 {
    (goal.row - pos.row).abs() + (goal.col - pos.col).abs()
}

fn compute_next_blizzards_occ(blizzards_occ : &mut BlizzardOccVec, blizzards : &mut Vec<Blizzard>, max_row : i16, max_col : i16) {
    let next = blizzards_occ.len();
    blizzards_occ.new_occ();

    for blizzard in blizzards {
        let mut pos = blizzard.pos;
        match blizzard.dir {
            b'>' => { pos.col += 1; if pos.col == max_col { pos.col = 1 } },
            b'v' => { pos.row += 1; if pos.row == max_row { pos.row = 1 } },
            b'<' => { pos.col -= 1; if pos.col == 0 { pos.col = max_col-1 } },
            b'^' => { pos.row -= 1; if pos.row == 0 { pos.row = max_row-1 } },
            _ => unreachable!(),
        }

        blizzard.pos = pos;
        blizzards_occ.set(next, pos.row, pos.col);
    }
}
