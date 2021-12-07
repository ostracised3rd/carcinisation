use std::collections::HashMap;
use crate::helpers::load_data;


struct Cache {
    memory: HashMap<i32, i32>
}

impl Cache {
    fn new() -> Self {
        Cache {
            memory: HashMap::new()
        }
    }

    fn calc(&mut self, num: i32) -> i32 {

        if self.memory.contains_key(&num) {
            return self.memory[&num]
        }

        (num * (num+1)) / 2
    }
}


pub fn run() {
    let data = load_data("data/aoc2021/d07.txt");
    let data = data.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    let res = p2(data);
    println!("{:?}", res);
}


fn probable_p1(data: Vec<i32>)  -> i32 {
    // is this correct??
    let mut p: HashMap<i32, i32> = HashMap::new(); 
    for i in data.iter() {
        *p.entry(*i).or_default() += 1;
    }

    let mut s = i32::MAX;
    for (k, v) in p.iter() {
        if v > &1 {
            s = s.min(data.iter().fold((0, k), | (acc, val), pos| {
                (acc + (pos - val).abs(), val)
            }).0);
        }
    }

    return s
}


fn p1(data: Vec<i32>) -> i32 {
    let mut s = i32::MAX;
    let mn = *data.iter().min().unwrap();
    let mx = *data.iter().max().unwrap();

    for k in mn..mx {
        s = s.min(data.iter().fold((0, k), | (acc, val), pos| {
            (acc + (pos - val).abs(), val)
        }).0);
    }

    return s
}


fn p2(data: Vec<i32>) -> i32 {
    let mut cache = Cache::new();
    let mut s = i32::MAX;

    let mid = (data.iter().sum::<i32>() as f32 / data.len() as f32).round() as i32;
    println!("{}", mid);

    for k in 100..(2*mid) {

        s = s.min(data.iter()
        .fold((0, k), | (acc, val), pos| {
            (acc + cache.calc((pos - val).abs()), val)
        }).0);
    }

    return s
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_efficient() {
        let data = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(37, p1(data));
    }

    #[test]
    fn most_efficient_linear_growth() {
        let data = vec![16,1,2,0,4,2,7,1,2,14];
        assert_eq!(168, p2(data));
    }
}

