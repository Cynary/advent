const INPUT: &str = include_str!("../input.txt");

fn main() {
    // input has been changed to only include moves.
    //
    let mut stack : [Vec<char>; 9] = [
        ['F', 'C', 'J', 'P', 'H', 'T', 'W'].to_vec(),
        ['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'].to_vec(),
        ['H', 'P', 'T', 'R'].to_vec(),
        ['Z', 'S', 'N', 'P', 'H', 'T'].to_vec(),
        ['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'].to_vec(),
        ['P', 'M', 'G', 'F', 'W', 'D', 'Z'].to_vec(),
        ['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'].to_vec(),
        ['N', 'D', 'S'].to_vec(),
        ['D', 'Z', 'S', 'F', 'M'].to_vec()
    ];

    let lines = INPUT.split('\n').filter(|l| !l.is_empty());
    for line in lines {
        let mut iter = line.trim().split(' ');
        _ = iter.next(); // move
        let count = iter.next().unwrap().parse::<usize>().expect("count");
        _ = iter.next(); // from
        let mut from = iter.next().unwrap().parse::<usize>().expect("from");
        _ = iter.next(); // to
        let mut to = iter.next().unwrap().parse::<usize>().expect("to");

        // for _ in 0..count {
        //     let c = stack[from-1].pop().unwrap();
        //     stack[to-1].push(c);
        // }

        let slice = stack[from-1].len() - count;
        let from_side : &mut [Vec<char>];
        let to_side : &mut [Vec<char>];

        if to > from {
            (from_side, to_side) = stack.split_at_mut(from);
            to -= from;
        } else {
            (to_side, from_side) = stack.split_at_mut(to);
            from -= to;
        }

        let (_, moving) = from_side[from-1].split_at_mut(slice);
        to_side[to-1].extend_from_slice(&moving);
        from_side[from-1].resize(slice, 'a');
    }

    for st in stack {
        print!("{}", st[st.len() - 1]);
    }
    println!("");
}
