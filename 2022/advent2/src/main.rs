use std::io::{self, BufRead};

fn shape(op : char, us : isize) -> isize
{
    let op = (op as isize) - ('A' as isize);
    match us
    {
        0 => (op + 2) % 3, // lose
        1 => op, // draw
        2 => (op + 1) % 3, // win
        _ => 0
    }
}

fn win(op : char, us : isize) -> bool
{
    match op
    {
        'A' => us == 1,
        'B' => us == 2,
        'C' => us == 0,
        _ => true
    }
}

fn draw(op : char, us : isize) -> bool
{
    match op
    {
        'A' => us == 0,
        'B' => us == 1,
        'C' => us == 2,
        _ => true
    }
}

fn main() -> io::Result<()>
{
    let mut lines = io::stdin().lock().lines();

    let mut score = 0;
    while let Some(line) = lines.next()
    {
        let input = line.unwrap();

        let mut split = input.split_whitespace();
        let op = split.next().unwrap().chars().nth(0).unwrap();
        let us = (split.next().unwrap().chars().nth(0).unwrap() as isize) - ('X' as isize);

        let us = shape(op, us);
        score += us+1;
        score += if win(op, us) { 6 } else if draw(op, us) { 3 } else { 0 };
    }

    println!("{}", score);

    Ok(())
}
