//use std::collections::HashMap;
fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let cv = code.chars().collect::<Vec<_>>();
    println!("{:?}", cv);

    let mut input = input;
    input.reverse();
    let mut res: Vec<u8> = Vec::new();
    let mut data: Vec<u8> = vec![0; 2];


    let mut p = 0usize;
    let mut i = 0usize;
    while let Some(&c) = cv.get(i) {
        println!("code point:{},code:{},data point:{},value:{}", i, c, p, data.get(p).unwrap());
        match c {
            '>' => {
                p += 1;
                if p >= data.len() {
                    data.push(0);
                }
            }
            '<' => {
                if p == 0 {
                    data.insert(0, 0);
                } else {
                    p -= 1;
                }
            }
            '+' => {
                if let Some(v) = data.get_mut(p) {
                    if *v == 255 {
                        *v = 0;
                    } else {
                        *v += 1;
                    }
                }
            }
            '-' => {
                if let Some(v) = data.get_mut(p) {
                    if *v == 0 {
                        *v = 255;
                    } else { *v -= 1; }
                }
            }
            '.' => {
                res.push(*data.get(p).unwrap());
                println!("res:{:?}", res)
            }
            ',' => {
                if let Some(v) = data.get_mut(p) {
                    *v = input.pop().unwrap();
                }
            }
            '[' => {
                if let Some(&v) = data.get(p) {
                    if v == 0 {
                        let mut n = 0u8;
                        while let Some(&c) = cv.get(i) {
                            if c == '[' {
                                n += 1;
                            } else if c == ']' {
                                if n == 1 {
                                    break;
                                } else {
                                    n -= 1;
                                }
                            }
                            i += 1;
                        }
                    }
                }
            }
            ']' => {
                if let Some(&v) = data.get(p) {
                    if v != 0 {
                        let mut n = 0u8;
                        while let Some(&c) = cv.get(i) {
                            if c == ']' {
                                n += 1;
                            } else if c == '[' {
                                if n == 1 {
                                    break;
                                } else {
                                    n -= 1;
                                }
                            }
                            i -= 1;
                        }
                    }
                }
            }
            _ => { println!("{}", c); }
        }
        i += 1;
    }
    println!("data:{:?}", data);
    res
}

fn ez_vec(s: &str, t: u8) -> Vec<u8> {

    let mut r = s.to_owned().into_bytes();
    r.push(t);
    r
}

fn main() {
    let code = ",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.";
    let input = vec![8u8, 9];

    let res = brain_luck(code, input);

    println!("res:{:?}", res);


    println!("ez_vec:{:?}", ez_vec("Codewars", 255));

    println!("res:{:?}", brain_luck(",[.[-],]", ez_vec("Codewars", 0)));
    println!("res:{:?}", brain_luck(",+[-.,+]", ez_vec("Codewars", 255)));
}
