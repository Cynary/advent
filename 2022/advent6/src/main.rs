use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut chars_so_far = HashMap::<char,usize>::new();
    let mut different_so_far = 0;
    let mut start_different = 0;

    for (i, c) in INPUT.trim().chars().enumerate() {
        match chars_so_far.get(&c) {
            None => {
                different_so_far += 1;
            }
            Some(x) => {
                if *x < start_different {
                    different_so_far += 1;
                } else {
                    different_so_far = i - x;
                    start_different = x+1;
                }
            }
        }
        chars_so_far.insert(c, i);

        if different_so_far == 14 {
            println!("{}", i+1);
            break;
        }
    }

    // let lines = INPUT.split('\n').filter(|l| !l.is_empty());
    // let mut count = 0;
    // let mut count2 = 0;

    // for line in lines
    // {
    //     let mut assignments = line.trim().split(',');
    //     let first = assignments.next().unwrap().split('-').map(|num| num.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    //     let second = assignments.next().unwrap().split('-').map(|num| num.parse::<isize>().unwrap()).collect();

    //     if contained(first, second) || contained(second, first)
    //     {
    //         count += 1;
    //     }

    //     if overlap(first, second)
    //     {
    //         count2 += 1;
    //     }
    // }

    // println!("part1: {}", count);
    // println!("part2: {}", count2);

    //     let line_array = line.trim().as_bytes();
    //     let rugsack_size = line_array.len()/2;
    //     assert!(line_array.len()%2 == 0);

    //     let first_rugsack = line_array.iter().enumerate().filter(|&(i, _)| i < rugsack_size).map(|(_, e)| e).collect::<HashSet<_>>();

    //     for i in rugsack_size..line_array.len()
    //     {
    //         // println!("{} {} {}", i, rugsack_size, line_array.len());
    //         let item_type = line_array[i];

    //         // println!("{}", item_type);
    //         assert!((item_type >= b'a' && item_type <= b'z') || (item_type >= b'A' && item_type <= b'Z'));
    //         if first_rugsack.contains(&item_type)
    //         {
    //             priority += (if item_type >= b'a' { item_type - b'a' + 1 } else { item_type - b'A' + 27 }) as isize;
    //             break;
    //         }
    //     }
    // }

    // println!("part1: {}", priority);

    // let lines = INPUT.split('\n');
    // priority = 0;
    // let mut badge_candidates = HashSet::<u8>::new();
    // for (i, sack) in lines.enumerate()
    // {
    //     let current_set = HashSet::<u8>::from_iter(sack.trim().as_bytes().iter().clone());
    //     if i % 3 == 0
    //     {
    //         badge_candidates = current_set;
    //     }
    //     else
    //     {
    //         badge_candidates = badge_candidates.intersection(&current_set).map(|e| *e).collect();
    //     }

    //     if i%3 == 2
    //     {
    //         assert!(badge_candidates.len() == 1);
    //         let item_type : u8 = *badge_candidates.iter().next().unwrap();
    //         priority += (if item_type >= b'a' { item_type - b'a' + 1 } else { item_type - b'A' + 27 }) as isize;
    //     }
    // }

    // println!("part2: {}", priority);
}
