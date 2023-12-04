use std::collections::{HashMap,HashSet,BinaryHeap};
use std::cmp::max;
const INPUT : &str = include_str!("../input.txt");

type CostValue = usize;

#[derive(Clone,Copy)]
struct Cost {
    ore : CostValue,
    clay : CostValue,
    obsidian : CostValue,
}

struct Blueprint {
    ore : Cost,
    clay : Cost,
    obsidian : Cost,
    geode : Cost,
}

impl Blueprint {
    pub fn new(line : &str) -> Self {
        let tokens : Vec<&str> = line.split(' ').collect();
        Self {
            ore : Cost { ore : tokens[6].parse::<CostValue>().unwrap(), clay : 0, obsidian : 0 },
            clay : Cost { ore : tokens[12].parse::<CostValue>().unwrap(), clay : 0, obsidian : 0},
            obsidian : Cost { ore : tokens[18].parse::<CostValue>().unwrap(), clay : tokens[21].parse::<CostValue>().unwrap(), obsidian : 0 },
            geode : Cost { ore : tokens[27].parse::<CostValue>().unwrap(), clay : 0, obsidian : tokens[30].parse::<CostValue>().unwrap() },
        }
    }
}

type CountRobots = usize;
type TimeValue = usize;

#[derive(Clone,Copy,Debug,Ord,PartialOrd,Eq,PartialEq,Hash)]
struct State {
    ore_robots : CountRobots,
    clay_robots : CountRobots,
    obsidian_robots : CountRobots,
    geode_robots : CountRobots,

    ore : CostValue,
    clay : CostValue,
    obsidian : CostValue,
    geode : CostValue,

    minute : TimeValue,
}

#[derive(PartialEq,Copy,Clone)]
enum RobotType {
    OreRobot,
    ClayRobot,
    ObsidianRobot,
    GeodeRobot,
    None
}

impl State {
    pub fn new() -> Self {
        Self {
            ore_robots : 1, clay_robots : 0, obsidian_robots : 0, geode_robots : 0,
            ore : 0, clay : 0, obsidian : 0, geode : 0,
            minute : 0
        }
    }

    fn build_and_produce(mut self, rt : RobotType, bp : &Blueprint, minutes : TimeValue) -> Option<Self> {
        // `self` is a copy, so it's fine to modify it directly.
        //
        self.minute += 1;

        let robot_cost = match &rt {
            &RobotType::OreRobot => bp.ore,
            &RobotType::ClayRobot => bp.clay,
            &RobotType::ObsidianRobot => bp.obsidian,
            &RobotType::GeodeRobot => bp.geode,
            &RobotType::None => Cost { ore : 0, clay : 0, obsidian : 0 },
        };

        // If we don't have enough resources to build the robot, then figure out how long it will
        // take to produce said resources, and bump the minutes by that amount.  If it is impossible
        // (because we'll run out the time, or don't have the pre-requisite robots), then we return
        // None here.
        //
        if self.ore < robot_cost.ore || self.clay < robot_cost.clay || self.obsidian < robot_cost.obsidian {
            let mut extra_time_needed = 0;
            if self.ore < robot_cost.ore {
                assert!(self.ore_robots >= 1);
                let remaining_ore_needed = robot_cost.ore-self.ore;
                extra_time_needed = remaining_ore_needed / self.ore_robots + if remaining_ore_needed  % self.ore_robots == 0 { 0 } else { 1 };
            }

            if  self.clay < robot_cost.clay {
                if self.clay_robots == 0 {
                    return None;
                }

                let remaining_clay_needed = robot_cost.clay-self.clay;
                extra_time_needed = max(
                    remaining_clay_needed / self.clay_robots + if remaining_clay_needed  % self.clay_robots == 0 { 0 } else { 1 },
                    extra_time_needed);
            }

            if  self.obsidian < robot_cost.obsidian {
                if self.obsidian_robots == 0 {
                    return None;
                }

                let remaining_obsidian_needed = robot_cost.obsidian-self.obsidian;
                extra_time_needed = max(
                    remaining_obsidian_needed / self.obsidian_robots + if remaining_obsidian_needed  % self.obsidian_robots == 0 { 0 } else { 1 },
                    extra_time_needed);
            }

            if extra_time_needed + self.minute > minutes {
                return None;
            }

            self.minute += extra_time_needed;
            self.ore += self.ore_robots*extra_time_needed;
            self.clay += self.clay_robots*extra_time_needed;
            self.obsidian += self.obsidian_robots*extra_time_needed;
            self.geode += self.geode_robots*extra_time_needed;
        }

        // Start build of robot.
        //
        self.ore -= robot_cost.ore;
        self.clay -= robot_cost.clay;
        self.obsidian -= robot_cost.obsidian;

        // Produce - we start building robot before producing, hence this specific order.
        //
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;

        // Finish building robot.
        //
        match rt {
            RobotType::OreRobot => self.ore_robots += 1,
            RobotType::ClayRobot => self.clay_robots += 1,
            RobotType::ObsidianRobot => self.obsidian_robots += 1,
            RobotType::GeodeRobot => self.geode_robots += 1,
            RobotType::None => {
                // If not building any robot, then just move the state to the end of time.
                //
                let rem = minutes-self.minute;
                self.minute = minutes;
                self.ore += self.ore_robots*rem;
                self.clay += self.clay_robots*rem;
                self.obsidian += self.obsidian_robots*rem;
                self.geode += self.geode_robots*rem;
            },
        }

        Some(self)
    }

    fn sensical(self, bp : &Blueprint) -> bool {
        let max_ore_cost = *[bp.ore.ore, bp.clay.ore, bp.obsidian.ore, bp.geode.ore].iter().max().unwrap();
        let max_clay_cost = bp.obsidian.clay;
        let max_obsidian_cost = bp.geode.obsidian;

        max_ore_cost >= self.ore_robots && max_clay_cost >= self.clay_robots && max_obsidian_cost >= self.obsidian_robots
    }
}

#[derive(Eq,Hash,PartialEq,Debug)]
struct StateHash {
    ore_robots : CountRobots,
    clay_robots : CountRobots,
    obsidian_robots : CountRobots,

    ore : CostValue,
    clay : CostValue,
    obsidian : CostValue,

    minute : TimeValue,
}

impl StateHash {
    pub fn new(state: &State) -> Self {
        Self {
            ore_robots : state.ore_robots,
            clay_robots : state.clay_robots,
            obsidian_robots : state.obsidian_robots,

            ore : state.ore,
            clay : state.clay,
            obsidian : state.obsidian,

            minute : state.minute
        }
    }
}

fn main() {
    let blueprints : Vec<Blueprint> = INPUT.trim().split('\n').map(|l| Blueprint::new(l)).collect();

    // let mut quality_sum = 0;
    // for (i, bp) in blueprints.iter().enumerate() {
    //     let quality = optimize2(24, &State::new(), bp);
    //     println!("Blueprint {i}: {quality}");
    //     quality_sum += (i+1) * quality;
    // }
    // println!("Part 1: {quality_sum}");

    let mut quality_mul = 1;
    for i in 0..3 {
        quality_mul *= optimize2(32, &State::new(), &blueprints[i]);
    }
    println!("Part 2: {quality_mul}");
}

fn optimize(minutes : TimeValue, state : &State, memo : &mut HashMap<StateHash, CostValue>, bp : &Blueprint) -> CostValue {
    let hash = StateHash::new(state);
    if let Some(ret) = memo.get(&hash) {
        return (minutes-state.minute) * state.geode_robots + state.geode + *ret;
    }

    // println!("{:?}", state);
    if state.minute == minutes {
        return state.geode;
    }

    let mut max_geode = 0;
    let mut some_robot_built = false;
    for rt in [RobotType::OreRobot, RobotType::ClayRobot, RobotType::ObsidianRobot, RobotType::GeodeRobot] {
        if let Some(next_state) = state.build_and_produce(rt, bp, minutes) {
            some_robot_built = true;
            max_geode = max(optimize(minutes, &next_state, memo, bp), max_geode);
        }
    }

    if (! some_robot_built) {
        let next_state = state.build_and_produce(RobotType::None, bp, minutes).unwrap();
        max_geode = max(max_geode, optimize(minutes, &next_state, memo, bp));
    }

    memo.insert(hash, max_geode-state.geode-((minutes-state.minute)*state.geode_robots));
    return max_geode;
}

fn optimize2(minutes : TimeValue, state : &State, bp : &Blueprint) -> CostValue {
    let mut q : BinaryHeap<(usize, State)> = BinaryHeap::new();
    let mut visited : HashSet<State> = HashSet::new();
    let mut max_geodes = 0;
    visited.insert(*state);
    q.push((heuristic(minutes, &state, &bp) + state.geode, *state));

    while let Some((_, state)) = q.pop() {
        if state.geode > max_geodes {
            max_geodes = state.geode;
        }

        if state.minute == minutes {
            max_geodes = state.geode;
            break;
        }
        else {
            let mut some_robot_built = false;
            for rt in [RobotType::OreRobot, RobotType::ClayRobot, RobotType::ObsidianRobot, RobotType::GeodeRobot, RobotType::None] {
                if let Some(next_state) = state.build_and_produce(rt, bp, minutes) {
                    if (some_robot_built && rt == RobotType::None) {
                        continue;
                    }

                    some_robot_built = true;
                    let max_possible_geodes = heuristic(minutes, &next_state, &bp);

                    if next_state.sensical(bp) && ! visited.contains(&next_state) {
                        visited.insert(next_state);
                        if max_possible_geodes > max_geodes {
                            q.push((max_possible_geodes, next_state));
                        }
                    }
                }
            }
        }
    }

    max_geodes
}

// over-estimate of the number of geodes.
//
fn heuristic(minutes : TimeValue, state : &State, bp: &Blueprint) -> CostValue{
    let mut state_opt : State = *state;
    let mut opt_ore = state_opt.ore;

    while state_opt.minute < minutes {
        opt_ore += state_opt.ore_robots;
        state_opt.geode += state_opt.geode_robots;

        if opt_ore >= bp.geode.ore && state_opt.obsidian >= bp.geode.obsidian {
            state_opt.geode_robots += 1;
            state_opt.obsidian -= bp.geode.obsidian;
        }

        state_opt.obsidian += state_opt.obsidian_robots;
        if opt_ore >= bp.obsidian.ore && state_opt.clay >= bp.obsidian.clay {
            state_opt.obsidian_robots += 1;
            state_opt.clay -= bp.obsidian.clay;
        }

        state_opt.clay += state_opt.clay_robots;
        if opt_ore >= bp.clay.ore {
            state_opt.clay_robots += 1;
        }

        let mut delta = 0;
        if state_opt.ore >= bp.ore.ore {
            state_opt.ore_robots += 1;
            state_opt.ore -= bp.ore.ore;
            delta = 1;
        }
        state_opt.ore += state_opt.ore_robots - delta;

        state_opt.minute += 1;
    }

    state_opt.geode

    // ---------------------------------------------------

    // how much ore can we get if we only build ore robots?
    //
    // let mut state_ore : State = *state;
    // while state_ore.minute != minutes {
    //     state_ore = match state_ore.build_and_produce(RobotType::OreRobot, bp, minutes) {
    //         Some(s) => s,
    //         None => state_ore.build_and_produce(RobotType::None, bp, minutes).unwrap(),
    //     }
    // }

    // // Since ore robots consume ore, the max ore is actually higher than the computed above.  The
    // // same is not true for other robots, though, since those robots don't consume their own
    // // resources.
    // //
    // state_ore.ore += (state_ore.ore_robots - state.ore_robots) * bp.ore.ore;

    // // Do the same thing for clay - find an overestimate by assuming we had the max possible ore from the beginning.
    // //
    // let mut state_clay : State = *state;
    // state_clay.ore = state_ore.ore;
    // state_clay.ore_robots = 0;
    // while state_clay.minute != minutes {
    //     state_clay = match state_clay.build_and_produce(RobotType::ClayRobot, bp, minutes) {
    //         Some(s) => s,
    //         None => state_clay.build_and_produce(RobotType::None, bp, minutes).unwrap(),
    //     }
    // }

    // // Same for obsidian.
    // //
    // let mut state_obsidian : State = *state;
    // state_obsidian.ore = state_ore.ore;
    // state_obsidian.ore_robots = 0;
    // state_obsidian.clay = state_clay.clay;
    // state_obsidian.clay_robots = 0;
    // while state_obsidian.minute != minutes+1 {
    //     state_obsidian = match state_obsidian.build_and_produce(RobotType::ObsidianRobot, bp, minutes) {
    //         Some(s) => s,
    //         None => state_obsidian.build_and_produce(RobotType::None, bp, minutes).unwrap(),
    //     }
    // }

    // // Finally, same for geode and return that.
    // //
    // let mut state_geode : State = *state;
    // state_geode.ore = state_ore.ore;
    // state_geode.ore_robots = 0;
    // state_geode.clay = state_clay.clay;
    // state_geode.clay_robots = 0;
    // state_geode.obsidian = state_obsidian.obsidian;
    // state_geode.obsidian_robots = 0;

    // while state_geode.minute != minutes {
    //     state_geode = match state_geode.build_and_produce(RobotType::GeodeRobot, bp, minutes) {
    //         Some(s) => s,
    //         None => state_geode.build_and_produce(RobotType::None, bp, minutes).unwrap(),
    //     }
    // }

    // state_geode.geode

    //----------------------------------------------
    // let rem = minutes-state.minute;
    // rem * (rem-1) / 2
}
