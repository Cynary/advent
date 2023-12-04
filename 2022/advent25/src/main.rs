const INPUT : &str = include_str!("../input.txt");

fn parse_num(l : &String) -> i64 {
    let mut num : i64 = 0;
    let mut mult : i64 = 1;
    for d in l.as_bytes().iter().rev() {
        num += match d {
            b'2' => 2*mult,
            b'1' => mult,
            b'0' => 0,
            b'-' => -1 * mult,
            b'=' => -2 * mult,
            _ => unreachable!(),
        };

        mult *= 5;
    }

    num
}

fn main() {
    let mut sum : i64 = 0;
    for l in INPUT.lines() {
        sum += parse_num(&l.to_string());
        // let mut num : i64 = 0;
        // let mut mult : i64 = 1;
        // for d in l.as_bytes().iter().rev() {
        //     num += match d {
        //         b'2' => 2*mult,
        //         b'1' => mult,
        //         b'0' => 0,
        //         b'-' => -1 * mult,
        //         b'=' => -2 * mult,
        //         _ => unreachable!(),
        //     };

        //     mult *= 5;
        // }
        // sum += num;
    }

    println!("{sum}");
    let sum_orig = sum;
    let mut digits : Vec<u8> = Vec::new();
    while sum != 0 {
        match sum%5 {
            4 => { digits.push(b'-'); sum += 5; },
            3 => { digits.push(b'='); sum += 5; },
            x => { digits.push(x as u8 + b'0'); }
        }
        sum /= 5
    }

    digits.reverse();
    let num_str = digits.iter().map(|b| *b as char).collect::<String>();
    assert!(sum_orig == parse_num(&num_str));
    println!("{num_str}");
}
