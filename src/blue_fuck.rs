fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let cv = code.chars().collect::<Vec<_>>();
    println!("{:?}", cv);

    let mut input = input;
    input.reverse();
    let mut cc = Vec::new();
    let mut res = Vec::new();
    let mut imm = Vec::new();
    let mut data = vec![false; 8];


    let mut p = 0usize;
    let mut i = 0usize;
    while let Some(&c) = cv.get(i) {
        match c {
            '>' => {
                p += 1;
                if p >= data.len() {
                    data.push(false);
                }
            }
            '<' => {
                if p == 0 {
                    data.insert(0, false);
                } else {
                    p -= 1;
                }
            }
            '+' => {
                if let Some(v) = data.get_mut(p) {
                    *v = !*v;
                }
            }
            ';' => {
                if imm.len() == 8 {
                    res.append(&mut imm);
                }
                imm.insert(0, *data.get(p).unwrap());
            }
            ',' => {
                if let Some(v) = data.get_mut(p) {
                    if cc.len() == 0 {
                        if let Some(ch) = input.pop() {
                            cc = from_bytes(&vec![ch]);
                        };
                    }
                    if let Some(b) = cc.pop() {
                        *v = b;
                    }
                }
            }
            '[' => {
                if let Some(&v) = data.get(p) {
                    if v == false {
                        let mut n = 1usize;
                        while n > 0 {
                            i += 1;
                            let c = *cv.get(i).unwrap();
                            if c == '[' {
                                n += 1;
                            } else if c == ']' {
                                n -= 1;
                            }
                        }
                    }
                }
            }
            ']' => {
                if let Some(&v) = data.get(p) {
                    if v == true {
                        let mut n = 1usize;
                        while n > 0 {
                            i -= 1;
                            let c = *cv.get(i).unwrap();
                            if c == ']' {
                                n += 1;
                            } else if c == '[' {
                                n -= 1;
                            }
                        }
                    }
                }
            }
            _ => { println!("{}", c); }
        }
        //println!("{}=>({}),dp:{},value:{}", i, c, p, data.get(p).unwrap());

        i += 1;
    }

    res.append(&mut imm);
    //println!("data:\n{:?}", data);
    //println!("res:\n{:?}", res);

    to_bytes(&res)
}

fn from_bytes(v: &Vec<u8>) -> Vec<bool> {
    let mut data: Vec<bool> = Vec::new();

    for n in v.iter() {
        for c in format!("{:08b}", n).chars() {
            match c {
                '0' => data.push(false),
                '1' => data.push(true),
                _ => {}
            }
        }
    }
    data
}

fn to_bytes(v: &Vec<bool>) -> Vec<u8> {

    v.chunks(8).map(|v| {
        let s: String = v.iter().map(|b| match *b {
            true => '1',
            false => '0'
        }).collect();

        u8::from_str_radix(&s, 2).unwrap()
    }).collect::<Vec<_>>()
}


#[test]
fn example_test_cases() {
    // Hello World Program taken from the official website
    assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
    // Echo until byte(0) encountered
    assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
    // Two numbers multiplier
    assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
}

