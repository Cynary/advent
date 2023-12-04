use std::collections::hash_map::HashMap;
use std::cell::{RefCell, RefMut};

const INPUT: &str = include_str!("../input.txt");

struct Directory {
    size : usize,
    subdirs : Vec<String>
}

fn parse_cmd(tokens : &Vec<&str>, current_directory : &mut String) {
    match tokens[1] {
        "cd" => {
            match tokens[2] {
               ".." => *current_directory = current_directory.get(..current_directory.rfind("/").unwrap()).unwrap().to_string(),
                _ => match tokens[2].as_bytes()[0] as char {
                    '/' => *current_directory = String::new(),
                    _ => {
                        current_directory.push_str("/");
                        current_directory.push_str(tokens[2]);
                    }
                }
            }
        }
        _ => {}
    }
}

fn compute_size(directories : &HashMap::<String,RefCell<Directory>>, mut current_dir : RefMut<Directory>) -> usize {
    while current_dir.subdirs.len() != 0 {
        let subname = current_dir.subdirs.pop().unwrap();
        let next_dir = directories[&subname].borrow_mut();
        // println!("{}, {}", subname, next_dir.size);
        current_dir.size += compute_size(&directories, next_dir);
    }
    current_dir.size
}

fn main() {
    let mut directories = HashMap::<String,RefCell<Directory>>::new();
    let mut current_directory = "".to_owned();

    for line in INPUT.trim().split('\n') {
        let tokens = line.trim().split(' ').collect::<Vec<&str>>();
        if tokens[0] == "$" {
                parse_cmd(&tokens, &mut current_directory);
        } else {
            if ! directories.contains_key(&current_directory) {
                directories.insert(current_directory.clone(), RefCell::new(Directory{size: 0, subdirs: Vec::new()}));
            }

            let mut d = directories.get(&current_directory).unwrap().borrow_mut();

            match tokens[0] {
                "dir" => {
                    let mut subdir = current_directory.clone();
                    subdir.push_str("/");
                    subdir.push_str(tokens[1]);
                    d.subdirs.push(subdir);
                }
                _ => d.size += tokens[0].parse::<usize>().unwrap(),
            }
        }
    }

    let mut total = 0;
    for (_n, d) in &directories {
        let current_dir = d.borrow_mut();
        let current_size = compute_size(&directories, current_dir);
        // println!("{}, {}", _n, current_size);
        if current_size <= 100000 {
            total += current_size;
        }
    }

    println!("{}", total);

    let total = directories[""].borrow().size;
    let needed = total - (70000000 - 30000000);
    let mut candidates = directories.values().map(|d| d.borrow().size).filter(|s| *s >= needed).collect::<Vec<usize>>();
    candidates.sort();
    println!("{}", candidates[0]);

    // let mut chars_so_far = HashMap::<char,usize>::new();
    // let mut different_so_far = 0;
    // let mut start_different = 0;

    // for (i, c) in INPUT.trim().chars().enumerate() {
    //     match chars_so_far.get(&c) {
    //         None => {
    //             different_so_far += 1;
    //         }
    //         Some(x) => {
    //             if *x < start_different {
    //                 different_so_far += 1;
    //             } else {
    //                 different_so_far = i - x;
    //                 start_different = x+1;
    //             }
    //         }
    //     }
    //     chars_so_far.insert(c, i);

    //     if different_so_far == 14 {
    //         println!("{}", i+1);
    //         break;
    //     }
    // }

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
