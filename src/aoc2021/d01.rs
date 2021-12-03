
use crate::helpers::load_data;


fn increase_counter(data: Vec::<i32>) -> i32 {
    data.iter()
        .fold((i32::MAX, 0), |(last, count), x| {
            (*x, if x>&last {count+1} else {count})
        }).1
}


fn three_sums(data: Vec::<i32>) -> Vec::<i32> {
    let mut sums: Vec::<i32> = Vec::new();
    
    for (i, x) in data.iter().enumerate() {
        if i+2 >= data.len() {break;}
        sums.push(x + data[i+1] + data[i+2] )
    }

    sums
}


pub fn run() {
    let data = data_parser();

    let data = three_sums(data);

    let count = increase_counter(data);
    println!("{}", count);
}

fn data_parser() -> Vec<i32> {
    let data = load_data("data/aoc2021/d01.txt");
    data.lines()
        .map(|x| x.trim().parse().unwrap())
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_check() {
       let data = vec![
        199,
        200,
        208,
        210,
        200,
        207,
        240,
        269,
        260,
        263,
       ];


       assert_eq!(7, increase_counter(data));
    }

    #[test]
    fn triple_sum() {
        let data = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];

        let res = vec![
            607,
            618,
            618,
            617,
            647,
            716,
            769,
            792,
        ];

        assert_eq!(res, three_sums(data));
    }
}