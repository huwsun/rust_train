use std::collections::HashMap;

fn part(n: isize, start: isize, av: &mut HashMap<String, isize>) -> isize {
    if n == 1 {
        1
    } else {
        let mut s = 0;
        let v = av.get(&format!("{}:{}", n, start)).cloned();
        //println!("{:?}", av);
        // println!("n:{},start:{},get_value:{:?}", n, start, v);

        match v {
            None => {
                for i in start..(n / 2 + 1) {
                    //println!("1:n:{},start:{},i:{},s:{}",n,start,i,s);

                    s += part(n - i, i, av) + 1;
                    //s+= part(n - i,i,av)+1 ;
                    // println!("2:n:{},start:{},i:{},s:{}", n, start, i, s);
                }
            }
            Some(x) => { s += x; }
        }

        av.entry(format!("{}:{}", n, start)).or_insert(s);

        println!("push n:{},start:{}, value:{}", n, start, s);
        //println!("{:?}", av);
        s
    }
}


fn partitions(n: isize) -> isize {
    let mut av = HashMap::new();
    let s = part(n, 1, &mut av);

    //println!("{:?}", av);
    s
}

fn part_1(n: isize, start: isize) -> isize {
    if n == 1 {
        1
    } else {
        let mut s = 0;
        for i in start..(n / 2 + 1) {
            s += part_1(n - i, i) + 1;
            println!("2:n:{},start:{},i:{},s:{}", n, start, i, s);
        }
        s
    }
}

fn partitions_1(n: isize) -> isize {
    part_1(n, 1)
}

//use std::collections::HashMap;

fn partitions_cached(n: isize, h: &mut HashMap<isize, isize>) -> isize {
    if n <= 1 {
        return 1;
    }
    if let Some(size) = h.get(&n) {
        println!("get n:{} from hashmap,size:{}", n, size);
        return *size;
    }
    let mut m = n - 1;
    let mut size = 0;
    let mut step = 2;
    while m >= 0 {
        //println!("1 => n:{},m:{},size:{},step:{}",n,m,size,step);
        let prev_size = partitions_cached(m, h);
        println!("2 => n:{},m:{},size:{},step:{},get prev_size:{}", n, m, size, step, prev_size);
        size += if (step / 2) % 2 == 1 { prev_size } else { prev_size * -1 };
        m -= if step % 2 == 1 { step } else { step / 2 };
        step += 1;

        println!("3 => n:{},m:{},size:{},step:{},prev_size:{}", n, m, size, step, prev_size);
    }
    h.insert(n, size);
    println!("insert n:{},size:{}", n, size);
    size
}

fn partitions_3(n: isize) -> isize {
    let mut h: HashMap<isize, isize> = HashMap::with_capacity(n as usize - 1);
    let v = partitions_cached(n, &mut h);
    println!("{:?}", h);
    v
}


fn factor(n: i64, s: &mut HashMap<i64, i64>) {
    let mut a = n.abs();
    let p = prime((a) as usize);
    println!("n:{},a:{},prime:{:?}", n, a, p);
    // println!("sqrt:{}", q);
    for i in &p {
        if a % i == 0 {
            println!("factor:{}", i);
            let v = s.get(i).cloned();
            match v {
                None => s.insert(*i, n),
                Some(x) => s.insert(*i, x + n)
            };
        }
    }
    println!("{:?}", s);
}

fn prime(n: usize) -> Vec<i64> {
    let mut p = vec![true; n + 1];
    p[0] = false;
    p[1] = false;
    for i in 2..n / 2 {
        if p[i] {
            let mut j = 2;
            while (i * j <= n) {
                p[i * j] = false;
                j += 1;
            }
        }
    }
    p.iter().enumerate().filter(|&(a, b)| *b).map(|(a, b)| a as i64)
        .collect()
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut s: HashMap<i64, i64> = HashMap::new();
    for i in &l {
        factor(*i, &mut s);
    }
    let mut r = s.iter().map(|(&a, &b)| (a, b)).collect::<Vec<_>>();
    //println!("r:{:?}", r);
    r.sort_by_key(|&(k, v)| k);

    // println!("r:{:?}", r);
    r
}

#[derive(Debug,PartialEq,Clone)]
enum Direction { NORTH, SOUTH, EAST, WEST }
use Direction::*;

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut r: Vec<Direction> = Vec::new();
    for d in arr.iter() {
        let d=d.clone();
        if let Some(p)=r.pop() {
            if (p == NORTH && d != SOUTH) || (p == SOUTH && d != NORTH) ||
                (p == EAST && d != WEST) || (p == WEST && d != EAST)
                {
                    r.push(p);
                    r.push(d);
                }
        } else{
            r.push(d);
        }

    }
    r
}