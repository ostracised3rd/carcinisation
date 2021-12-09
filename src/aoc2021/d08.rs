use std::{collections::HashMap, ops::RangeBounds};

use crate::helpers::load_data;


pub fn run() {
    let data = load_data("data/aoc2021/d08.txt");
    // println!("{}", p1(&data));
    println!("{}", p2(&data));
}


fn p1(data: &str) -> usize {
    data.lines()
        .map(|s| {
            s.split_once(" | ")
             .unwrap()
             .1
             .split_whitespace()
             .map(|n| n.chars().count())
             .filter(|x| [2, 3, 4, 7].contains(x))
             .collect::<Vec<usize>>()
        })
        .flatten()
        .collect::<Vec<usize>>()
        .len()
}


fn numbers(data: &str) -> Vec<&str> {
    data.split_whitespace()
        .collect::<Vec<&str>>()
}


fn data_parser(data: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    data.lines()
        .map(|s| {
            let (a, b) = s.split_once(" | ").unwrap();
            (numbers(a), numbers(b))
        })
        .collect::<Vec<(Vec<&str>, Vec<&str>)>>()
}



struct Decoder {
    one: Vec<char>,
    four: Vec<char>,
    not_in_nine: char,
}

impl Decoder {
    fn new() -> Self {
        Decoder {
            four: Vec::new(),
            one: Vec::new(),
            not_in_nine: ' ',
        }
    }

    fn find_not_in_nine(&mut self, candidate: Vec<char>) {
        for c in &self.four {
            if !candidate.contains(&c) {
                return
            }
        }

        let possible = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

        for i in possible {
            if !candidate.contains(&i) {
                self.not_in_nine = i;
            }
        }
    }


    fn fives(&self, cs: Vec<char>) -> &'static str {
        for c in &self.one {
            if !cs.contains(&c) {
                if cs.contains(&self.not_in_nine) {
                    return "2"
                } else {
                    return "5"
                }
            }
        }

        "3"
    } 
    
    fn sixes(&self, cs: Vec<char>) -> &'static str {
        if !cs.contains(&self.not_in_nine) {
            return "9"
        }

        for c in &self.one {
            if !cs.contains(&c) {
                return "6"
            }
        }

        "0"
    }
}


fn wires(data: Vec<&str>) -> Decoder {
    let mut decoder = Decoder::new();
    let mut sixes: Vec<Vec<char>> = Vec::new();
    for n in data {
        let num: Vec<char> = n.chars().collect();
        match num.len() {
            2 => decoder.one = num,
            4 => decoder.four = num,
            6 => sixes.push(num),
            _ => continue
        } 
    } 

    for i in sixes {
        decoder.find_not_in_nine(i);
    }
    

    decoder
}


fn decode(decoder: &Decoder, num: &str) -> &'static str {
    let cs: Vec<char> = num.chars().collect();
    match cs.len() {
        2 => "1",
        3 => "7",
        4 => "4",
        7 => "8",
        5 => {let res = decoder.fives(cs); return res;}, 
        6 => {let res = decoder.sixes(cs); return res;}, 
        _ => ""
    } 
}


fn p2(data: &str) -> i32 {
    let data = data_parser(data);
    println!("{:?}", data);

    let mut numbers = Vec::<i32>::new();
    for datum in data {
        let decoder = wires(datum.0);
        let mut number = Vec::<String>::new();
        for digit in datum.1 {
            number.push(String::from(decode(&decoder, digit)));
        }
        println!("{:?}", number);
        numbers.push(number.join("").parse().unwrap())
    }

    println!("{:?}", numbers);

    return numbers.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_num_count() {
        let data = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(26, p1(data));
    }


    #[test]
    fn sum_of_numbers() {
        let data = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

        assert_eq!(61229, p2(data));
    }
}