use crate::helpers::load_data;
use std::collections::{HashMap, hash_map::Entry};
use std::ops::Add;

fn data_parser(data: &str) -> Vec<((i32, i32), (i32, i32))> {
    data.lines()
        .map(|x| {
            let d = x.split(" -> ")
                .map(|y| { 
                    let c =y.split(",")
                        .map(|z| z.parse().unwrap())
                        .collect::<Vec<i32>>();
                    (c[0], c[1])
                })
                .collect::<Vec<(i32, i32)>>();
            (d[0], d[1])
        })
        .collect::<Vec<((i32, i32), (i32, i32))>>()
}


fn straight_lines(data: Vec<((i32, i32), (i32, i32))>) -> HashMap<(i32, i32), i32> {
    let mut nodes: HashMap<(i32, i32), i32> = HashMap::new();
    
    for ((x1,y1), (x2, y2)) in data {
        if x1 == x2 {
            let (y1, y2) = if y1 > y2 {(y2, y1)} else {(y1, y2)};
            for y in y1..=y2 {
                *nodes.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            let (x1, x2) = if x1 > x2 {(x2, x1)} else {(x1, x2)};
            for x in x1..=x2 {
                *nodes.entry((x, y1)).or_insert(0) +=1;
            }
        }
    }

    nodes
}


fn lining(data: Vec<((i32, i32), (i32, i32))>) -> HashMap<(i32, i32), i32> {
    let mut nodes: HashMap<(i32, i32), i32> = HashMap::new();
    
    for ((x1,y1), (x2, y2)) in data {
        if x1 == x2 {
            let (y1, y2) = if y1 > y2 {(y2, y1)} else {(y1, y2)};
            for y in y1..=y2 {
                *nodes.entry((x1, y)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            let (x1, x2) = if x1 > x2 {(x2, x1)} else {(x1, x2)};
            for x in x1..=x2 {
                *nodes.entry((x, y1)).or_insert(0) +=1;
            }
        } else {
            let xm = i32::signum(x2-x1);
            let ym = i32::signum(y2-y1);

            for i in 0..=((y2-y1)*ym) {
                *nodes.entry((x1+(xm*i), y1+(ym*i))).or_insert(0) +=1;
            }
        }
    }

    nodes
}


fn p1(data: &str) -> u32 {
    let res = data_parser(data);
    let res = straight_lines(res);
    res.iter().fold(0, |c, x| if x.1 > &1 {c+1} else {c})
}


fn p2(data: &str) -> u32 {
    let res = data_parser(data);
    let res = lining(res);
    res.iter().fold(0, |c, x| if x.1 > &1 {c+1} else {c})
}


pub fn run() {
    let data = load_data("data/aoc2021/d05.txt");
    
    // let res = p1(&data);
    let res = p2(&data);
    println!("{}", res);

}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn overlapping() {
        let data = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(5, p1(data));
    }

    #[test]
    fn p2_overlapping() {
        let data = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        assert_eq!(12, p2(data));
    }
}