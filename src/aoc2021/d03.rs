
use crate::helpers::load_data;


fn conversion(data: Vec<u32>) -> u32 {
    data.iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x * u32::pow(2, i as u32))
}


fn population(data: &str) -> (u32, Vec<u32>) {
    data_parser(data)
        .iter()
        .fold((0, Vec::<u32>::new()), |(i, acc), x| {
            if acc.len() == 0 {return (i+1, x.clone())}
            (i+1, x.iter().zip(acc).map(|(i, j)| i+j).collect())
        })
}


fn p1(length: u32, data: Vec<u32>) -> u32 {
    let mut gamma = Vec::<u32>::new();
    let mut epsilon = Vec::<u32>::new();
    let mid = length / 2;

    for i in data {
        if i > mid {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    }

    conversion(gamma) * conversion(epsilon)
}


fn data_parser(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|x| x.chars().map(|y| if y == '1' {1} else {0}).collect::<Vec<u32>>())
        .collect()
}


fn index_pop(index: usize, data: &Vec<Vec<u32>>) -> usize {
    data.iter()
        .fold(0, |size, x| size + x[index]) as usize
}


fn filter_pop(index: usize, val: u32, data: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    data.iter()
        .filter(|x| x[index] == val)
        .map(|x| x.clone())
        .collect::<Vec<Vec<u32>>>()
}


fn chemical_value(val: (u32, u32), index: usize, data: Vec<Vec<u32>>) -> Vec<u32> {
    if data.len() == 1 {
        return data[0].clone();
    }

    let mid = (data.len() as f32 / 2.).ceil() as usize;
    let pop = index_pop(index, &data);
    let keep = if pop >= mid {val.0} else {val.1};

    return chemical_value(val, index+1, filter_pop(index, keep, data));
}


fn p2(data: &str) -> u32 {
    let data = data_parser(data);
    let oxy = chemical_value((1, 0), 0, data.clone());
    let co2 = chemical_value((0, 1), 0, data);
    
    conversion(oxy) * conversion(co2)
}


pub fn run() {
    let data = load_data("data/aoc2021/d03.txt");
    // let (len, pop) = population(&data);
    // let res = p1(len, pop);
    let res = p2(&data);
    println!("{}", res);
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn population_check() {
        let data = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let res = (12, vec![7, 5, 8, 7, 5]);
        assert_eq!(res, population(data));
    }


    #[test]
    fn binary_to_decimal() {
        let data = vec![1, 0, 1, 0, 1, 0];
        assert_eq!(42, conversion(data));
    }


    #[test]
    fn power_consumption() {
        let data = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let data = population(data);
        assert_eq!(198, p1(data.0, data.1));
    }


    #[test]
    fn oxygen_generator() {
        let data = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let data = data_parser(data);
        assert_eq!(vec![1,0,1,1,1], chemical_value((1, 0), 0, data));
    }


    #[test]
    fn co2_scrubber() {
        let data = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        let data = data_parser(data);
        assert_eq!(vec![0,1,0,1,0], chemical_value((0, 1), 0, data));
    }

    #[test]
    fn life_support() {
        let data = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

        assert_eq!(230, p2(data));
    }
}