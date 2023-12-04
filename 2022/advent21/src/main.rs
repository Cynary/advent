use std::collections::HashMap;

const INPUT : &str = include_str!("../input.txt");

#[derive(Debug,Clone)]
struct Operation {
    monkey1 : String,
    monkey2 : String,
    op : u8,
}

impl Operation {
    pub fn new(items : &mut dyn Iterator<Item=&str>) -> Self {
        let monkey1 = items.next().unwrap().to_string();
        let op = items.next().unwrap().as_bytes()[0];
        let monkey2 = items.next().unwrap().to_string();

        Self { monkey1, monkey2, op }
    }

    fn compute(&self, monkey_map : &HashMap<String, Monkey>) -> Option<Monkey> {
        if let Monkey::Num(monkey1_val) = monkey_map[&self.monkey1] {
            if let Monkey::Num(monkey2_val) = monkey_map[&self.monkey2] {
                return match self.op {
                    b'+' => Some(Monkey::Num(monkey1_val + monkey2_val)),
                    b'-' => Some(Monkey::Num(monkey1_val - monkey2_val)),
                    b'*' => Some(Monkey::Num(monkey1_val * monkey2_val)),
                    b'/' => Some(Monkey::Num(monkey1_val / monkey2_val)),
                    _ => unreachable!(),
                }
            }
        }

        None
    }

#[derive(Debug)]
enum Monkey {
    Num(isize),
    Op(Operation),
    Nothing,
}

impl Monkey {
    fn Num(&self) -> isize {
        if let Monkey::Num(val) = self { *val } else { unreachable!() }
    }

    fn Op(&self) -> Operation {
        if let Monkey::Op(val) = self { val.clone() } else { unreachable!() }
    }
}

fn solve(name : &String, mm : &mut HashMap<String, Monkey>) {
    while (! matches!(mm[&mm[name].Op().monkey1], Monkey::Num(_))) && (! matches!(mm[&mm[name].Op().monkey2], Monkey::Num(_))) {
        let mut results : Vec<(String, Monkey)> = Vec::new();
        for (name, m) in mm.iter() {
            if let Monkey::Op(op) = m {
                if let Some(mon) = op.compute(&mm) {
                    results.push((name.clone(), mon));
                }
            }
        }

        for (name, val) in results {
            mm.insert(name, val);
        }
    }
}

fn main() {
    let mut mm : HashMap<String, Monkey> = HashMap::new();
    for l in INPUT.lines() {
        let mut tokens = l.split(' ').peekable();
        let name = tokens.next().unwrap().trim_matches(':');
        if name == "humn" {
            mm.insert(name.to_string(), Monkey::Nothing);
            continue;
        }

        if let Ok(num) = tokens.peek().unwrap().parse::<isize>() {
            mm.insert(name.to_string(), Monkey::Num(num));
        }
        else {
            mm.insert(name.to_string(), Monkey::Op(Operation::new(&mut tokens)));
        }
    }

    println!("Solving for root");
    solve(&"root".to_string(), &mut mm);
    println!("Done solving for root");

    let mut name : String = String::new();
    let mut val : isize = 0;
    if let Monkey::Num(v) = mm[&mm["root"].Op().monkey1] {
        val = v;
        name = mm["root"].Op().monkey2.clone();
    }
    else if let Monkey::Num(v) = mm[&mm["root"].Op().monkey2] {
        val = v;
        name = mm["root"].Op().monkey1.clone();
    }

    loop {
        if name == "humn" {
            println!("{val}");
            break;
        }

        if let Monkey::Num(v) = mm[&mm[&name].Op().monkey1] {
            let next_name = mm[&name].Op().monkey2.clone();
            let next_val = match mm[&name].Op().op {
                    b'+' => val - v,
                    b'-' => v - val,
                    b'*' => val / v,
                    b'/' => v / val,
                    _ => unreachable!(),
            };

            mm.insert(name, Monkey::Num(val));
            name = next_name;
            val = next_val;
        }
        else if let Monkey::Num(v) = mm[&mm[&name].Op().monkey2] {
            let next_name = mm[&name].Op().monkey1.clone();
            let next_val = match mm[&name].Op().op {
                    b'+' => val - v,
                    b'-' => val + v,
                    b'*' => val / v,
                    b'/' => v * val,
                    _ => unreachable!(),
            };

            mm.insert(name, Monkey::Num(val));
            name = next_name;
            val = next_val;
        }
        else {
            println!("Solving for {name}");
            solve(&name, &mut mm);
        }
    }
}
