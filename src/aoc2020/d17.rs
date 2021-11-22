

// data parsers
fn root(x: &str) -> Vec<i32> {
    x.chars()
     .map(|i| if i == '#' {1} else {0})
     .collect::<Vec<i32>>()
}


fn squared(y: &str) -> Vec<Vec<i32>> {
    y.lines()
     .map(|x| root(x))
     .collect::<Vec<Vec<i32>>>()
}


fn cubed(z: &str) -> Vec<Vec<Vec<i32>>> {
    z.split("\n\n")
     .map(|y| squared(y))
     .collect::<Vec<Vec<Vec<i32>>>>()
}


fn hypercube(w: &str) -> Vec<Vec<Vec<Vec<i32>>>> {
    w.split("\n\n")
     .map(|y| cubed(y))
     .collect::<Vec<Vec<Vec<Vec<i32>>>>>()
}

// p1
fn others(x: &i32, y: &i32, z: &i32, data: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut res = 0;

    for zp in z-1..=z+1 {
        if zp < 0 || zp >= data.len() as i32 {
            // println!("z:{} l:{}", &zp, data.len());
            continue;
        } 
        for yp in y-1..=y+1 {
            if yp < 0 || yp >= data[0].len() as i32 {
                // println!("z:{} y:{} l:{}", &zp, &yp, data[0].len());
                continue;
            } 
            for xp in x-1..=x+1 {
                if xp < 0 || xp >= data[0][0].len() as i32 {
                    // println!("z:{} y:{} x:{}, l:{}", &zp, &yp, &xp, data[0][0].len());
                    continue;
                }

                // println!("access: {} {} {}", &zp, &yp, &xp);
                res += data[zp as usize][yp as usize][xp as usize];
            }
        }
    }

    res
}


fn next_gen(data: &Vec<Vec<Vec<i32>>>)  ->  Vec<Vec<Vec<i32>>> {
   
    let mut z_axis : Vec<Vec<Vec<i32>>> = Vec::new();
    for z in -1..=data.len() as i32  {

        let mut y_axis: Vec<Vec<i32>> = Vec::new();
        for y in -1..=data[0].len() as i32  {

            let mut x_axis: Vec<i32> = Vec::new();
            for x in -1..=data[0][0].len() as i32 {

                let cell = 
                if  (z < 0 || z >= data.len() as i32) || 
                    (y < 0 || y >= data[0].len() as i32) ||
                    (x < 0 || x >= data[0][0].len() as i32 ) {
                        0
                } else {
                    data[z as usize][y as usize][x as usize]
                };
                
                // println!("corr: z{} y{} x{} c{}", &z, &y, &x, &cell);
                let other = others(&x, &y, &z, data);
                // println!("other: {}", &other);

                let cell = if (cell == 1 && [3, 4].contains(&other)) ||
                                (cell == 0 && other == 3) {1} else {0};

                x_axis.push(cell);
            }
            y_axis.push(x_axis);
        }
        z_axis.push(y_axis);
    }

    z_axis
}


fn its_alive(data: &Vec<Vec<Vec<i32>>>) -> i32 {
    data.iter()
        .map(|y| y.iter()
                                .map(|x| x.iter().sum::<i32>())
                                .sum::<i32>())
        .sum::<i32>()
}


// p2
fn p2_others(x: &i32, y: &i32, z: &i32, w: &i32, data: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    let mut res = 0;

    for wp in w-1..=w+1 {
        if wp < 0 || wp >= data.len() as i32 {
            // println!("z:{} l:{}", &zp, data.len());
            continue;
        } 
        for zp in z-1..=z+1 {
            if zp < 0 || zp >= data[0].len() as i32 {
                // println!("z:{} l:{}", &zp, data.len());
                continue;
            } 
            for yp in y-1..=y+1 {
                if yp < 0 || yp >= data[0][0].len() as i32 {
                    // println!("z:{} y:{} l:{}", &zp, &yp, data[0].len());
                    continue;
                } 
                for xp in x-1..=x+1 {
                    if xp < 0 || xp >= data[0][0][0].len() as i32 {
                        // println!("z:{} y:{} x:{}, l:{}", &zp, &yp, &xp, data[0][0].len());
                        continue;
                    }
    
                    // println!("access: {} {} {}", &zp, &yp, &xp);
                    res += data[wp as usize][zp as usize][yp as usize][xp as usize];
                }
            }
        }
    }
    
    res
}


fn p2_next_gen(data: &Vec<Vec<Vec<Vec<i32>>>>)  ->  Vec<Vec<Vec<Vec<i32>>>> {
   
    let mut w_axis : Vec<Vec<Vec<Vec<i32>>>> = Vec::new();
    for w in -1..=data.len() as i32  {

        let mut z_axis : Vec<Vec<Vec<i32>>> = Vec::new();
        for z in -1..=data[0].len() as i32  {

            let mut y_axis: Vec<Vec<i32>> = Vec::new();
            for y in -1..=data[0][0].len() as i32  {

                let mut x_axis: Vec<i32> = Vec::new();
                for x in -1..=data[0][0][0].len() as i32 {

                    let cell = 
                    if  (w < 0 || w >= data.len() as i32) || 
                        (z < 0 || z >= data[0].len() as i32) ||
                        (y < 0 || y >= data[0][0].len() as i32) ||
                        (x < 0 || x >= data[0][0][0].len() as i32 ) {
                            0
                    } else {
                        data[w as usize][z as usize][y as usize][x as usize]
                    };
                    
                    // println!("corr: z{} y{} x{} c{}", &z, &y, &x, &cell);
                    let other = p2_others(&x, &y, &z, &w, &data);
                    // println!("other: {}", &other);

                    let cell = if (cell == 1 && [3, 4].contains(&other)) ||
                                    (cell == 0 && other == 3) {1} else {0};

                    x_axis.push(cell);
                }
                y_axis.push(x_axis);
            }
            z_axis.push(y_axis);
        }
        w_axis.push(z_axis);
    }

    w_axis
}


fn p2_its_alive(data: &Vec<Vec<Vec<Vec<i32>>>>) -> i32 {
    data.iter()
        .map(|z| 
            z.iter()
            .map(|y| 
                y.iter()
                .map(|x| x.iter().sum::<i32>())
                .sum::<i32>())
            .sum::<i32>())
        .sum::<i32>()
}



pub fn run() {
//     let data = ".#.
// ..#
// ###";

    let data = load_data();
    let p2 = hypercube(data);
    println!("{:?}", p2);


    // let p1 = cubed(data);
    // println!("c0:{}", its_alive(&p1));

    let c1 = p2_next_gen(&p2);
    println!("{:?}", &c1);
    println!("c1:{}", p2_its_alive(&c1));

    let c2 = p2_next_gen(&c1);
    println!("c2:{}", p2_its_alive(&c2));

    let c3 = p2_next_gen(&c2);
    println!("c3:{}", p2_its_alive(&c3));

    let c4 = p2_next_gen(&c3);
    println!("c4:{}", p2_its_alive(&c4));

    let c5 = p2_next_gen(&c4);
    println!("c5:{}", p2_its_alive(&c5));

    let c6 = p2_next_gen(&c5);
    println!("c6:{}", p2_its_alive(&c6));
}


fn load_data() -> &'static str {
"#....#.#
..##.##.
#..#..#.
.#..#..#
.#..#...
##.#####
#..#..#.
##.##..#"
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_parsed_data() {
        let data = ".#.
..#
###";

        let res = vec![
            vec![
                vec![0,1,0],
                vec![0,0,1],
                vec![1,1,1],
            ],
        ];

        // [
        //     [
        //         [
        //             [0, 1, 0], 
        //             [0, 0, 1], 
        //             [1, 1, 1]
        //         ]
        //     ]
        // ];


        assert_eq!(res, cubed(data));
    }


    #[test]
    fn cycle1() {
        let data = ".#.
..#
###";

        let res = vec![
            vec![
                vec![0,0,0,0,0],
                vec![0,0,0,0,0],
                vec![0,1,0,0,0],
                vec![0,0,0,1,0],
                vec![0,0,1,0,0]
            ],
            vec![
                vec![0,0,0,0,0],
                vec![0,0,0,0,0],
                vec![0,1,0,1,0],
                vec![0,0,1,1,0],
                vec![0,0,1,0,0]
            ],
            vec![
                vec![0,0,0,0,0],
                vec![0,0,0,0,0],
                vec![0,1,0,0,0],
                vec![0,0,0,1,0],
                vec![0,0,1,0,0]
            ]
        ];


        assert_eq!(res, next_gen(&cubed(data)))
    }
}
