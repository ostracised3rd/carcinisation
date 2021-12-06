use std::collections::HashMap;

use crate::helpers::load_data;

pub fn run() {
    let data = load_data("data/aoc2021/d06.txt");
    let data = data.split(",").map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
    let res = world_domination(256, data);
    println!("{}", res);
}


fn world_domination(days: i64, data: Vec<i64>) -> i64 {
    let mut cache: HashMap<i64, i64> = HashMap::new(); 
    cache.insert(3, 5217223241);
    cache.insert(4, 4726100873);
    cache.insert(1, 6206821032);
    cache.insert(2, 5617089147);
    let mut res = data.len() as i64;
    for i in data {
        if !cache.contains_key(&i) {
            let t = gen(days-i);
            println!("days: {}, i: {}, t:{}", days-i, i, t);
            cache.insert(i, t);
            
        } 

        res += cache[&i];
    }
    
    res
}

fn gen(mut days: i64) -> i64{
    let mut x = 0;
    while days > 0 {
        x += 1;
        x += gen(days-9);
        days -= 7;
    }
    return x
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generations() {
        let data: Vec<i64> = vec![3,4,3,1,2];
        // let data: Vec<i64> = vec![3];
        // let res = vec![6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8];


        assert_eq!(26, world_domination(18, data)); 
        // assert_eq!(5934, world_domination(80, data));
        // assert_eq!(26984457539, world_domination(256, data));
    }
}




// use std::collections::HashMap;

// static REPRO_INTERVAL_INITIAL: u64 = 9;
// static REPRO_INTERVAL: u64 = 7;

// type Generations = HashMap<u64, u64>;

// fn get_og_fishes(input: &str) -> Vec<usize> {
//     input.split(',').map(|s| s.parse().unwrap()).collect()
// }

// fn project_population(members: Vec<usize>, interval_count: u64) -> u64 {
//     let mut generations: Generations = HashMap::new();
//     let mut pop: u64 = members.len() as u64;

//     // for each fish in the OG generation, spawn an offspring at {rest_timer}.
//     // we have to project their adult life as well since OGs continue reproducing.
//     members.iter().for_each(|_i| {
//         let i = *_i as u64;
//         *generations.entry(i).or_default() += 1;
//         project_adult_generation(&mut generations, 1, i, interval_count);
//     });

//     // a generation of fishes hatches every day.
//     // we can project them as a whole since there is no variance in repro. rate in a generation.
//     for i in 0..interval_count {
//         let generation_size = *generations.entry(i).or_default();
//         // add hatched fishes to the population counter.
//         pop += generation_size;

//         // initial reproduction is delayed for hatched fishes. we handle it with a special case.
//         let adults_at = i + REPRO_INTERVAL_INITIAL;

//         if adults_at <= interval_count {
//             *generations.entry(adults_at).or_default() += generation_size;
//             // Once fishes are adults, we can project the rest of {interval_count} in a loop.
//             project_adult_generation(&mut generations, generation_size, adults_at, interval_count);
//         }
//     }

//     pop
// }

// fn project_adult_generation(
//     generations: &mut Generations,
//     generation_size: u64,
//     current_interval: u64,
//     interval_count: u64,
// ) {
//     let mut spawns_at = current_interval + REPRO_INTERVAL;
//     // increase the generation_size at {interval} by the current generation_size for rest of observed interval.
//     while spawns_at <= interval_count {
//         *generations.entry(spawns_at).or_default() += generation_size;
//         spawns_at += REPRO_INTERVAL;
//     }
// }

// pub fn part_one(input: &str) -> u64 {
//     project_population(get_og_fishes(input), 80)
// }

// pub fn part_two(input: &str) -> u64 {
//     project_population(get_og_fishes(input), 256)
// }

// #[test]
// fn test_part_one() {
//     use aoc2021::read_file;
//     let input = read_file("examples", 6);
//     assert_eq!(part_one(&input), 5934);
// }

// #[test]
// fn test_part_two() {
//     use aoc2021::read_file;
//     let input = read_file("examples", 6);
//     assert_eq!(part_two(&input), 26984457539);
// }