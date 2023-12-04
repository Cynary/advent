const INPUT : &str = include_str!("../input.txt");

// const WORRY_REDUCER : usize = 3;
// const ROUNDS : usize = 20;

const WORRY_REDUCER : usize = 1;
const ROUNDS : usize = 10000;

enum Operation {
    Add,
    Multiply,
}

enum Modifier {
    Number(usize),
    WorryLevel,
}

struct WorryModifier {
    operation : Operation,
    modifier : Modifier,
}

impl WorryModifier {
    pub fn new(to_parse : &str) -> Self {
        let mut tokens = to_parse.trim().split(' ');
        assert!("Operation:".eq(tokens.next().unwrap()));
        assert!("new".eq(tokens.next().unwrap()));
        assert!("=".eq(tokens.next().unwrap()));
        assert!("old".eq(tokens.next().unwrap()));

        let op_str = tokens.next().unwrap();
        let operation = match op_str { "*" => Operation::Multiply, "+" => Operation::Add, _ => unreachable!() };

        let modifier_str = tokens.next().unwrap();
        let modifier = match modifier_str {
            "old" => Modifier::WorryLevel,
            x => Modifier::Number(x.parse::<usize>().unwrap()),
        };

        Self { operation: operation, modifier: modifier }
    }

    fn apply(&self, worry_level : usize, divisible_by : usize) -> usize {
        let modifier = match self.modifier {
            Modifier::WorryLevel => worry_level,
            Modifier::Number(x) => x,
        };

        match self.operation {
            Operation::Add => ((worry_level + modifier) / WORRY_REDUCER) % divisible_by,
            Operation::Multiply => ((worry_level * modifier) / WORRY_REDUCER) % divisible_by,
        }
    }
}

struct MonkeyThrowTest {
    divisible_by : usize,
    throw_on_false : usize,
    throw_on_true : usize,
}

impl MonkeyThrowTest {
    pub fn new<'a>(mut test_lines : impl Iterator<Item=&'a str>) -> Self {
        let mut test = test_lines.next().unwrap().trim().split(' ');
        assert!("Test:".eq(test.next().unwrap()));
        assert!("divisible".eq(test.next().unwrap()));
        assert!("by".eq(test.next().unwrap()));
        let divisible_by = test.next().unwrap().parse::<usize>().unwrap();

        let mut throw_on_true = 0;
        let mut throw_on_false = 0;
        for condition in test_lines {
            let mut tokens = condition.trim().split(' ');
            assert!("If".eq(tokens.next().unwrap()));
            let case = tokens.next().unwrap();
            assert!("throw".eq(tokens.next().unwrap()));
            assert!("to".eq(tokens.next().unwrap()));
            assert!("monkey".eq(tokens.next().unwrap()));
            match case {
                "true:" => throw_on_true = tokens.next().unwrap().parse::<usize>().unwrap(),
                "false:" => throw_on_false = tokens.next().unwrap().parse::<usize>().unwrap(),
                _ => unreachable!()
            }
        }

        Self { divisible_by: divisible_by, throw_on_false: throw_on_false, throw_on_true: throw_on_true }
    }

    fn apply(&self, worry_level : usize) -> usize {
        match worry_level % self.divisible_by {
            0 => self.throw_on_true,
            _ => self.throw_on_false
        }
    }
}

struct Monkey {
    _id : usize,
    items : Vec<usize>,
    worry_modifier : WorryModifier,
    throw : MonkeyThrowTest,
}

struct Throw {
    worry : usize,
    throw_to : usize,
}

impl Monkey {
    pub fn new(to_parse : &str, divisible_by : &mut usize) -> Self {
        let mut lines = to_parse.trim().split('\n');

        // Get id
        //
        let mut monkey_id = lines.next().unwrap().trim().split(' ');
        assert!("Monkey".eq(monkey_id.next().unwrap()));
        let id = monkey_id.next().unwrap().replace(':', "").parse::<usize>().unwrap();

        // Get starting items:
        //
        let mut items = lines.next().unwrap().trim().split(' ');
        assert!("Starting".eq(items.next().unwrap()));
        assert!("items:".eq(items.next().unwrap()));
        let items_vec = items.map(|i| i.replace(',', "").parse::<usize>().unwrap()).collect();

        let worry_modifier = WorryModifier::new(lines.next().unwrap());
        let throw = MonkeyThrowTest::new(lines);
        *divisible_by *= throw.divisible_by;

        return Self {
            _id: id,
            items: items_vec,
            worry_modifier: worry_modifier,
            throw: throw
        }
    }

    fn turn(&self, divisible_by : usize) -> Vec<Throw> {
        self.items.iter().map(|worry| {
            let new_worry = self.worry_modifier.apply(*worry, divisible_by);
            Throw {worry: new_worry, throw_to: self.throw.apply(new_worry) }
        }).collect()
    }
}

fn main() {
    let mut divisible_by : usize = 1;
    let mut monkeys : Vec<Monkey> = INPUT.trim().split("\n\n").map(|monkey_str| Monkey::new(monkey_str, &mut divisible_by)).collect();
    let mut monkey_inspections = vec![0usize; monkeys.len()];
    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            for throw in monkeys[i].turn(divisible_by) {
                monkey_inspections[i] += 1;
                assert!(throw.throw_to != i);
                monkeys[throw.throw_to].items.push(throw.worry);
            }
            monkeys[i].items.clear();
        }
    }

    monkey_inspections.sort();
    monkey_inspections.reverse();
    println!("{}", monkey_inspections[0] * monkey_inspections[1]);
}
