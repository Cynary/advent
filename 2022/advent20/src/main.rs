use std::collections::VecDeque;

const INPUT : &str = include_str!("../input.txt");

fn main() {
    let key = 811589153;
    let iters = 10;
    let nums : Vec<isize> = INPUT.lines().map(|l| l.parse::<isize>().unwrap()*key).collect();
    let indexes : Vec<usize> = (0..nums.len()).collect();
    let len = nums.len();
    println!("{len}");

    let mut indexes_fast : SqrtVec = SqrtVec::new(indexes);
    for _ in 0..iters {
        for i in 0..len {
            let current_index = indexes_fast.position(i);
            let next_index = ((((current_index as isize + nums[i]) % (len-1) as isize) + (len-1) as isize) % (len-1) as isize) as usize;
            indexes_fast.mov(current_index, next_index);
        }
    }
    let start = indexes_fast.position(nums.iter().position(|e| *e == 0).unwrap());
    let num1000 = nums[indexes_fast.at((start+1000)%len)];
    let num2000 = nums[indexes_fast.at((start+2000)%len)];
    let num3000 = nums[indexes_fast.at((start+3000)%len)];
    let coord = num1000 + num2000 + num3000;
    println!("{num1000} {num2000} {num3000} {start} {coord}");
}

struct SqrtVec {
    segments : usize,
    list : Vec<VecDeque<usize>>,
    segments_map : Vec<usize>,
}

impl SqrtVec {
    pub fn new(v : Vec<usize>) -> Self {
        let segments = 2*(v.len() as f64).sqrt().ceil() as usize;
        let mut segments_map = Vec::new();
        // let mut list : Vec<LinkedList<usize>> = vec![LinkedList::new(); segments];
        let mut list : Vec<VecDeque<usize>> = vec![VecDeque::new(); segments];

        for (i, val) in v.iter().enumerate() {
            list[i / segments].push_back(*val);
            segments_map.push(i/segments);
        }

        Self { segments, list, segments_map }
    }

    fn at(&self, index : usize) -> usize {
        *self.list[index / self.segments].iter().nth(index % self.segments).unwrap()
    }

    fn position(&self, value : usize) -> usize {
        let segment = self.segments_map[value];
        segment * self.segments + self.list[segment].iter().position(|v| *v == value).unwrap()
    }

    fn mov(&mut self, from : usize, to : usize) {
        let f = from / self.segments;
        let s = to / self.segments;

        // Remove element from its spot on first segment.
        //
        let v = self.list[f].remove(from%self.segments).unwrap();

        if from < to {
            for i in f..s {
                let v = self.list[i+1].pop_front().unwrap();
                self.list[i].push_back(v);
                self.segments_map[v] = i;
            }
        }
        else {
            for i in s..f {
                let v = self.list[i].pop_back().unwrap();
                self.list[i+1].push_front(v);
                self.segments_map[v] = i+1;
            }
        }

        // Add element to its spot on second segment.
        //
        self.list[s].insert(to%self.segments, v);
        self.segments_map[v] = s;
    }

    fn _render(&self, nums : &Vec<isize>) {
        for ll in &self.list {
            print!("{:?}", *ll);
        }
        println!("");

        for ll in &self.list {
            for i in ll {
                print!("{}, ", nums[*i]);
            }
        }
        println!("");
        println!("");
    }
}
