use std::collections::{HashSet, BinaryHeap};
const INPUT : &str = include_str!("../input.txt");

#[derive(PartialEq,Eq,Hash,Copy,Clone,Ord,PartialOrd,Debug)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    pub fn new(line : &str) -> Self {
        let v = line.trim().split(',').map(|s| s.parse::<isize>().unwrap()).collect::<Vec<isize>>();
        Self { x : v[0], y : v[1], z : v[2] }
    }

    fn sides(self: &Cube) -> Vec<Cube> {
        let mut r = Vec::new();
        for d in [-1, 1] {
            r.push(Cube { x : self.x+d, y : self.y  , z : self.z   });
            r.push(Cube { x : self.x  , y : self.y+d, z : self.z   });
            r.push(Cube { x : self.x  , y : self.y  , z : self.z+d });
        }

        r
    }

    fn mh(self: &Cube, other: &Cube) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

fn main() {
    let cubes = INPUT.trim().split('\n').map(|l| Cube::new(l)).collect::<HashSet<Cube>>();
    let mut shared_sides = 0;
    let mut unreachable_sides = 0;

    for cube in &cubes {
        for side in cube.sides() {
            if cubes.contains(&side) {
                shared_sides += 1;
            }
            else if ! side_reachable(&side, &cubes) {
                unreachable_sides += 1;
            }
        }
    }

    println!("part1: {}", cubes.len()*6-shared_sides);
    println!("part2: {}", cubes.len()*6-(shared_sides + unreachable_sides));
}

fn side_reachable(side: &Cube, cubes: &HashSet<Cube>) -> bool {
    let mut visited : HashSet<Cube> = HashSet::new();
    let mut q : BinaryHeap<(isize, isize, Cube)> = BinaryHeap::new();
    let goal = Cube{x:-1,y:-1,z:-1};
    visited.insert(*side);
    q.push((-side.mh(&goal), 0, *side));

    while let Some((_, cost, cube)) = q.pop() {
        if cube == goal {
            return true;
        }

        for side in cube.sides() {
            if !visited.contains(&side) && !cubes.contains(&side) {
                visited.insert(side);
                q.push((-(side.mh(&goal) + cost + 1), cost+1, side));
            }
        }
    }

    false
}
