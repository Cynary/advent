use std::collections::{HashSet, HashMap};
const INPUT : &str = include_str!("../input.txt");

type Elf = (isize, isize); // (row, col)

fn n(elf : Elf) -> Elf { (elf.0-1, elf.1) }
fn ne(elf : Elf) -> Elf { (elf.0-1, elf.1+1) }
fn e(elf : Elf) -> Elf { (elf.0, elf.1+1) }
fn se(elf : Elf) -> Elf { (elf.0+1, elf.1+1) }
fn s(elf : Elf) -> Elf { (elf.0+1, elf.1) }
fn sw(elf : Elf) -> Elf { (elf.0+1, elf.1-1) }
fn w(elf : Elf) -> Elf { (elf.0, elf.1-1) }
fn nw(elf : Elf) -> Elf { (elf.0-1, elf.1-1) }

fn adjacent(elf : Elf) -> Vec<Elf> {
    vec![n(elf), ne(elf), e(elf), se(elf), s(elf), sw(elf), w(elf), nw(elf)]
}

fn render(elves : &HashSet<Elf>) {
    let rmin = *elves.iter().map(|(r,_)| r).min().unwrap();
    let rmax = *elves.iter().map(|(r,_)| r).max().unwrap();
    let cmin = *elves.iter().map(|(_,c)| c).min().unwrap();
    let cmax = *elves.iter().map(|(_,c)| c).max().unwrap();

    for r in rmin..=rmax {
        for c in cmin..=cmax {
            print!("{}", if elves.contains(&(r,c)) { '#' } else { '.' });
        }
        println!("");
    }
    println!("");
}

fn main() {
    let mut elves : HashSet<Elf> = HashSet::new();

    for (i, l) in INPUT.lines().enumerate() {
        for (j, _) in l.match_indices('#') {
            elves.insert((i as isize, j as isize));
        }
    }

    for i in 0.. {
        // println!("Round {i}");
        // render(&elves);

        if i == 10 {
            // Part 1
            //
            let rmin = elves.iter().map(|(r,_)| r).min().unwrap();
            let rmax = elves.iter().map(|(r,_)| r).max().unwrap()+1;
            let cmin = elves.iter().map(|(_,c)| c).min().unwrap();
            let cmax = elves.iter().map(|(_,c)| c).max().unwrap()+1;

            println!("{}, {}", elves.len(), (rmax-rmin) * (cmax-cmin) - elves.len() as isize);
        }

        let mut elf_next_pos : Vec<(Elf, Elf)> = Vec::new();
        let mut proposed_pos : HashMap<Elf, usize> = HashMap::new();

        // first half
        //
        let mut no_move = true;
        for elf in elves.iter().cloned() {
            if adjacent(elf).iter().all(|e| !elves.contains(e)) {
                // No adjacent elves, no movement proposal.
                //
                continue;
            }
            no_move = false;

            for dir in 0..4 {
                let d = (dir + i%4)%4;
                let (a, b, c) = match d {
                    0 => (nw(elf), n(elf), ne(elf)),
                    1 => (sw(elf), s(elf), se(elf)),
                    2 => (nw(elf), w(elf), sw(elf)),
                    3 => (ne(elf), e(elf), se(elf)),
                    _ => unreachable!(),
                };

                if !elves.contains(&a) && !elves.contains(&b) && !elves.contains(&c) {
                    elf_next_pos.push((elf, b));
                    proposed_pos.insert(b, proposed_pos.get(&b).unwrap_or(&0) + 1);
                    break;
                }
            }
        }

        if no_move {
            // Part 2
            //
            println!("Stopped moving at round {}", i+1);
            break;
        }

        // second half
        //
        for (prev, next) in elf_next_pos {
            if proposed_pos[&next] == 1 {
                elves.remove(&prev);
                elves.insert(next);
            }
        }
        // elves = elf_next_pos.iter().map(|(prev,next)| if prev == next || proposed_pos[next] != 1 { *prev } else { *next }).collect();
    }
}
