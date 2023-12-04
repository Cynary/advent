use std::collections::{HashMap,VecDeque,BinaryHeap};
use std::cmp::max;

const INPUT : &str = include_str!("../input.txt");

fn main() {
    let mut valve_count = 0u16;
    let mut rvalves : HashMap<String, u16> = HashMap::new();
    let mut neighbors0 : Vec<Vec<String>> = Vec::new();
    let mut flows0 : Vec<i16> = Vec::new();

    for line in INPUT.trim().split('\n') {
        let mut tokens = line.split(' ');
        assert!("Valve".eq(tokens.next().unwrap()));

        let valve = tokens.next().unwrap();
        rvalves.insert(valve.to_string(), valve_count);

        assert!("has".eq(tokens.next().unwrap()));
        assert!("flow".eq(tokens.next().unwrap()));

        let rate = tokens.next().unwrap().split(['=', ';']).collect::<Vec<&str>>()[1].parse::<i16>().unwrap();
        flows0.push(rate);

        tokens.next();
        tokens.next();
        assert!("to".eq(tokens.next().unwrap()));
        tokens.next();

        let mut n = Vec::new();
        while let Some(x) = tokens.next() {
            let mut nvalve = x.to_string();

            if nvalve.as_bytes()[nvalve.len()-1] == b',' {
                nvalve.pop();
            }

            n.push(nvalve);
        }
        neighbors0.push(n);
        valve_count += 1;
    }

    assert!(usize::from(valve_count) == neighbors0.len());
    assert!(usize::from(valve_count) == flows0.len());
    assert!(usize::from(valve_count) == rvalves.len());

    let neighbors : Vec<Vec<u16>> = neighbors0.iter().map(|nv| nv.iter().map(|s| rvalves[s]).collect()).collect();
    let costs0 : Vec<Vec<i16>> = (0..valve_count).map(|i| bfs(i, &neighbors)).collect();

    let mut start_valve = 0u16;
    let mut valve_map : Vec<u16> = Vec::new();
    let mut rvalve_map : Vec<u16> = Vec::new();
    let mut new_valve = 0u16;

    for v in 0..valve_count {
        if flows0[usize::from(v)] != 0 || v == rvalves["AA"] {
            valve_map.push(new_valve);
            rvalve_map.push(v);

            if v == rvalves["AA"] {
                start_valve = new_valve;
            }

            new_valve += 1;
        }
        else {
            valve_map.push(valve_count);
        }
    }

    // don't use rvalves or neighbors after this.
    //
    let costs : Vec<Vec<i16>> = costs0.iter().cloned().enumerate()
        .filter(|(v, _)| valve_map[*v] != valve_count)
        .map(|(_, c)| c.iter().cloned().enumerate()
             .filter(|(v, _)| valve_map[*v] != valve_count)
             .map(|(_, c)| c)
             .collect())
        .collect();

    let flows : Vec<i16> = flows0.iter().enumerate()
        .filter(|(v, _)| valve_map[*v] != valve_count)
        .map(|(_, f)| *f)
        .collect();

    valve_count = new_valve;
    assert!(costs.len() == usize::from(valve_count));
    assert!(flows.len() == usize::from(valve_count));
    for (i, f) in flows.iter().enumerate() {
        assert!(*f == flows0[usize::from(rvalve_map[i])]);
    }

    for (i, c) in costs.iter().enumerate() {
        assert!(c.len() == usize::from(valve_count));
        for (j, cc) in c.iter().enumerate() {
            assert!(*cc == costs0[usize::from(rvalve_map[i])][usize::from(rvalve_map[j])]);
        }
    }

    println!("{}", bfs2(&costs, &flows, valve_count, start_valve));
    println!("{}", astar(&costs, &flows, valve_count, start_valve));
}

fn bfs(valve : Valve, neighbors : &Vec<Vec<Valve>>) -> Vec<i16> {
    let mut q : VecDeque<(Valve, i16)> = VecDeque::new();
    let sentinel = -1;
    let mut h : Vec<i16> = vec![sentinel; neighbors.len()];
    q.push_back((valve, 0));
    h[usize::from(valve)] = 0;

    while let Some((v, c)) = q.pop_front() {
        for n in &neighbors[usize::from(v)] {
            if h[usize::from(*n)] == sentinel {
                q.push_back((*n, c+1));
                h[usize::from(*n)] = c+1;
            }
        }
    }

    h
}

type Valve = u16;
type Minutes = i16;
type ReleasedPressure = i16;

fn bfs2(costs : &Vec<Vec<i16>>, flows : &Vec<i16>, valves : Valve, start_valve : Valve) -> i16 {
    let mut q : VecDeque<(Valve, Minutes, ReleasedPressure, VisitedSet)> = VecDeque::new();
    let mut max_released_pressure = 0;

    let mut visited = VisitedSet::new(valves);
    visited.insert(start_valve);
    q.push_back((start_valve, 30, 0, visited));
    while let Some((valve, minutes, release, visited)) = q.pop_front() {
        if release > max_released_pressure {
            max_released_pressure = release;
        }

        for nvalve in 0..valves {
            let next_minutes = minutes - costs[usize::from(valve)][usize::from(nvalve)] - 1;
            if visited.contains(nvalve) || next_minutes <= 0 {
                continue;
            }

            let mut next_visited = visited.clone();
            next_visited.insert(nvalve);
            let next_release = release + next_minutes * flows[usize::from(nvalve)];
            q.push_back((nvalve, next_minutes, next_release, next_visited));
        }
    }

    max_released_pressure
}

#[derive(Clone, PartialOrd, PartialEq, Eq, Ord)]
struct VisitedSet {
    bitset : u32,
}

impl VisitedSet {
    pub fn new(len : u16) -> Self {
        assert!(len < 32);
        Self { bitset : 0 }
    }

    fn insert(&mut self, v : Valve) {
        // let bitindex = usize::from(v / 32);
        let bitoffset = usize::from(v % 32);
        self.bitset |= 1u32 << bitoffset;
    }

    fn contains(&self, v : Valve) -> bool {
        // let bitindex = usize::from(v / 32);
        let bitoffset = usize::from(v % 32);

        0 != (self.bitset & (1u32 << bitoffset))
    }
}

fn astar(costs : &Vec<Vec<i16>>, flows : &Vec<i16>, valves : Valve, start_valve : Valve) -> i16 {
    let mut q : BinaryHeap<(Potential, Valve, Valve, Minutes, Minutes, ReleasedPressure, VisitedSet)> = BinaryHeap::new();
    let mut max_released_pressure = 0;

    let mut visited = VisitedSet::new(valves);
    visited.insert(start_valve);
    let mins = 26i16;
    q.push((potential_pressure(valves, &flows, &visited, mins), start_valve, start_valve, mins, mins, 0, visited));
    while let Some((_, valve1, valve2, minutes1, minutes2, release, visited)) = q.pop() {
        if release > max_released_pressure {
            max_released_pressure = release;
            println!("{max_released_pressure}");
        }

        for mut nvalve in 0..valves {
            for mut nvalve2 in 0..valves {
                let mut next_minutes1 = minutes1 - costs[usize::from(valve1)][usize::from(nvalve)] - 1;
                let mut next_minutes2 = minutes2 - costs[usize::from(valve2)][usize::from(nvalve2)] - 1;

                if nvalve == nvalve2 || visited.contains(nvalve) || visited.contains(nvalve2) || (next_minutes1 <= 0 && next_minutes2 <= 0) {
                    continue;
                }

                let mut flow1 = flows[usize::from(nvalve)];
                let mut flow2 = flows[usize::from(nvalve2)];
                if next_minutes1 <= 0 {
                    next_minutes1 = minutes1;
                    nvalve = valve1;
                    flow1 = 0;
                }

                if next_minutes2 <= 0 {
                    next_minutes2 = minutes2;
                    nvalve2 = valve2;
                    flow2 = 0;
                }

                let mut next_visited = visited.clone();
                next_visited.insert(nvalve);
                next_visited.insert(nvalve2);
                let next_release = release + next_minutes1 * flow1 + next_minutes2 * flow2;
                let potential = next_release + potential_pressure(valves, &flows, &next_visited, max(next_minutes1, next_minutes2));

                if potential < max_released_pressure {
                    continue;
                }

                q.push((potential, nvalve, nvalve2, next_minutes1, next_minutes2, next_release, next_visited));
            }
        }
    }

    max_released_pressure
}

type Potential = i16;

// Returns an upper bound on the maximum amount of extra pressure that can still be released.
//
fn potential_pressure(valves : Valve, flows : &Vec<i16>, visited : &VisitedSet, remaining_mins : i16) -> Potential {
    (0..valves).filter(|v| !visited.contains(*v)).map(|v| flows[usize::from(v)] * (remaining_mins - 2)).sum()
}
