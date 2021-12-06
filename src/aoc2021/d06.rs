use std::collections::HashMap;

use crate::helpers::load_data;

pub fn run() {
    let data = load_data("data/aoc2021/d06.txt");
    let data = data.split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let res = thats_a_rotate(18, &data);
    println!("{}", res);
}


fn thats_a_rotate(days: u32, data: &Vec<usize>) -> i64 {
    let mut gens = Vec::<i64>::from([0; 9]);
    
    for i in data {
        gens[*i] += 1;
    }

    for i in 0..days {
        gens.rotate_left(1);
        gens[6] += gens[8];
    }
    
    gens.iter().sum()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generations() {
        let data: Vec<usize> = vec![3,4,3,1,2];

        assert_eq!(26, thats_a_rotate(18, &data)); 
        assert_eq!(5934, thats_a_rotate(80, &data));
        assert_eq!(26984457539, thats_a_rotate(256, &data));
    }
}
