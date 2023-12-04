use std::cmp::Ordering;
const INPUT : &str = include_str!("../input2.txt");

fn main() {
    let mut packets : Vec<Element> = Vec::new();
    let mut val = 0;
    for (i, block) in INPUT.trim().split("\n\n").enumerate() {
        let mut packet_strs = block.split('\n');

        let first = parse(packet_strs.next().unwrap());
        let second = parse(packet_strs.next().unwrap());

        if cmp(&first, &second) == Ordering::Less {
            val += i+1;
        }

        packets.push(first);
        packets.push(second);
    }
    println!("{val}");

    let mut dividers = vec![parse("[[2]]"), parse("[[6]]")];
    packets.extend(dividers.iter().cloned());

    dividers.sort_by(cmp);
    packets.sort_by(cmp);

    let mut val2 = 1;
    let mut di = 0;
    for (i, p) in packets.iter().enumerate() {
        if cmp(p, &dividers[di]) == Ordering::Equal {
            val2 *= i+1;
            println!("Divider {di} is in position: {}", i+1);
            di += 1;
            if di == dividers.len() {
                break;
            }
        }
    }

    println!("{val2}");
}

fn cmp(first: &Element, second: &Element) -> Ordering {
    match (first, second) {
        (Element::Number(x), Element::Number(y)) => x.cmp(y),
        (Element::Number(_), Element::List(y)) => cmp_list(&vec![first.clone()], y),
        (Element::List(x), Element::Number(_)) => cmp_list(x, &vec![second.clone()]),
        (Element::List(x), Element::List(y)) => cmp_list(x, y),
    }
}

fn cmp_list(first : &Vec<Element>, second : &Vec<Element>) -> Ordering {
    let mut f = first.iter();
    let mut s = second.iter();

    loop {
        match (f.next(), s.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(x), Some(y)) => match cmp(x, y) { Ordering::Equal => (), o => return o },
        }
    }
}

#[derive(Clone, Debug)]
enum Element {
    Number(isize),
    List(Vec<Element>),
}

#[derive(PartialEq)]
enum Token {
    StartList,
    EndList,
    Number(isize),
}

fn lexer(s : &str) -> Vec<Token> {
    assert!(b'[' == s.as_bytes()[0]);
    let mut r : Vec<Token> = Vec::new();
    _ = lexer_list(s, 0, &mut r);
    return r;
}

fn lexer_list(s : &str, mut index : usize, r : &mut Vec<Token>) -> usize {
    assert!(b'[' == s.as_bytes()[index]);
    r.push(Token::StartList);
    index += 1;
    while index < s.len() {
        match s.as_bytes()[index] {
            b'[' => index = lexer_list(s, index, r),
            b']' => {
                r.push(Token::EndList);
                index += 1;
                break;
            }
            x if x.is_ascii_digit() => index = lexer_num(s, index, r),
            b',' => index += 1,
            _ => unreachable!(),
        }
    }

    index
}

fn lexer_num(s : &str, mut index : usize, r : &mut Vec<Token>) -> usize {
    let mut current_val = 0isize;
    while index < s.len() && s.as_bytes()[index].is_ascii_digit() {
        current_val = current_val*10 + (s.as_bytes()[index] - b'0') as isize;
        index += 1;
    }

    r.push(Token::Number(current_val));
    index
}

fn parse(s : &str) -> Element {
    parse_list(&lexer(s), &mut 0)
}

fn parse_list(tokens : &Vec<Token>, index : &mut usize) -> Element {
    let mut r : Vec<Element> = Vec::new();
    assert!(tokens[*index] == Token::StartList);
    *index += 1;
    loop {
        match tokens[*index] {
            Token::StartList => r.push(parse_list(tokens, index)),
            Token::EndList => {
                *index += 1;
                return Element::List(r);
            }
            Token::Number(x) => {
                r.push(Element::Number(x));
                *index += 1;
            }
        }
    }
}
