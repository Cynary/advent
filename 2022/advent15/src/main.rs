use std::collections::HashSet;
use std::cmp::{max, min};
const INPUT : &str = include_str!("../input.txt");

fn main() {
    let mut beacons = HashSet::<(isize, isize)>::new();
    let mut sensors = HashSet::<(isize, isize, isize)>::new();
    for sensor in INPUT.trim().split('\n') {
        let mut tokens = sensor.trim().split(' ');
        assert!("Sensor".eq(tokens.next().unwrap()));
        assert!("at".eq(tokens.next().unwrap()));

        let xs = parse_pos(tokens.next().unwrap());
        let ys = parse_pos(tokens.next().unwrap());

        assert!("closest".eq(tokens.next().unwrap()));
        assert!("beacon".eq(tokens.next().unwrap()));
        assert!("is".eq(tokens.next().unwrap()));
        assert!("at".eq(tokens.next().unwrap()));

        let x = parse_pos(tokens.next().unwrap());
        let y = parse_pos(tokens.next().unwrap());
        beacons.insert((x, y));
        sensors.insert((xs, ys, (x-xs).abs() + (y-ys).abs()));
    }

    println!("{}", count_impossible(&sensors, &beacons, 10));
    // println!("{}", count_impossible(&sensors, &beacons, 2000000));

    for y in 0..=20 {
        let (found, pos) = beacon_in_y(&sensors, y, 0, 20);
        if found {
            println!("{pos}");
            break;
        }
    }

    for y in 0..=4000000 {
        let (found, pos) = beacon_in_y(&sensors, y, 0, 4000000);
        if found {
            println!("{pos}");
            break;
        }
    }
}

fn parse_pos(pos : &str) -> isize {
    return pos.split(['=', ',', ':']).collect::<Vec<&str>>()[1].parse::<isize>().unwrap();
}

fn count_impossible(sensors: &HashSet<(isize, isize, isize)>, beacons: &HashSet<(isize, isize)>, y : isize) -> isize {
    let mut ranges : Vec<(isize, isize)> = sensors.iter()
        .filter(|(_, ys, dist)| (ys - y).abs() <= *dist)
        .map(|(xs, ys, dist)| {
            let spread = dist - (ys - y).abs();
            (xs-spread, xs+spread)
        }).collect();

    ranges.sort();
    let mut min_x = ranges[0].0;
    let mut prev_x = ranges[0].1;
    let mut count = 0;

    println!("{:?}", ranges);
    for i in 1..ranges.len() {
        if prev_x+1 < ranges[i].0 {
            count += prev_x - min_x + 1;
            min_x = ranges[i].0;
        }

        prev_x = max(ranges[i].1, prev_x);
    }

    count += prev_x - min_x + 1;

    count - (beacons.iter().filter(|(_, yb)| *yb == y).count() as isize)
}

fn beacon_in_y(sensors: &HashSet<(isize, isize, isize)>, y : isize, rmin : isize, rmax : isize) -> (bool, isize) {
    let mut ranges : Vec<(isize, isize)> = sensors.iter()
        .filter(|(xs, ys, dist)| {
            if (ys - y).abs() <= *dist {
                let spread = dist - (ys - y).abs();
                ! (xs+spread < rmin || xs - spread > rmax)
            } else {
                false
            }
        })
        .map(|(xs, ys, dist)| {
            let spread = dist - (ys - y).abs();
            (max(xs-spread, rmin), min(xs+spread, rmax))
        }).collect();

    ranges.sort();
    // println!("{y}, {:?}", ranges);

    if ranges[0].0 > rmin {
        return (true, rmin * 4000000 + y);
    } else if ranges.iter().cloned().map(|(_,y)| y).max().unwrap() < rmax {
        return (true, rmax * 4000000 + y);
    }

    let mut prev_x = ranges[0].1;

    for i in 1..ranges.len() {
        if prev_x+1 < ranges[i].0 {
            // println!("{prev_x} {:?}", ranges[i]);
            // Found a solution.
            //
            return (true, (prev_x+1)*4000000 + y);
        }

        prev_x = max(ranges[i].1, prev_x);
    }

    return (false, 0);
}
