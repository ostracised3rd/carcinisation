use crate::helpers::load_data;


#[derive(Debug, Clone)]
struct Block {
    num: u32,
    called: bool,
}


impl Block {
    fn new(num: u32) -> Self {
        Block {
            num,
            called: false,
        }
    }

    fn calling(&mut self) {
        self.called = true
    }
}


#[derive(Debug, Clone)]
struct Board {
    blocks: Vec<Vec<Block>>,
    row_size: u32,
    col_size: u32,
    row_rates: Vec<u32>,
    col_rates: Vec<u32>,
}


impl Board {
    fn new(data: &str) -> Self {
        let blocks = data.lines()
            .map(|x| {
                x.split_whitespace()
                 .map(|b| Block::new(b.parse().unwrap()))
                 .collect::<Vec<Block>>()
            })
            .collect::<Vec<Vec<Block>>>();

        let row_size = blocks[0].len() as u32;
        let col_size= blocks.len() as u32;

        Board {
            blocks,
            row_size,
            col_size,
            row_rates: (0..row_size).map(|_| 0).collect(),
            col_rates: (0..col_size).map(|_| 0).collect(),
        }
    }

    fn is_bingo(&mut self, num: u32) -> bool {
        for (i, row) in self.blocks.iter_mut().enumerate() {
            for (j, b) in row.iter_mut().enumerate() {
                if b.num == num {
                    b.calling();
                    self.row_rates[j] += 1;
                    self.col_rates[i] += 1;

                    if self.row_rates[j] == self.row_size || self.col_rates[i] == self.col_size {
                        return true
                    }
                }
            }
        }

        false
    }

    fn count(&self) -> u32 {
        let mut c = 0;
        for row in self.blocks.iter() {
            for b in row.iter() {
                if !b.called {
                    c += b.num;
                }
            }
        }

        c
    }
}


fn gen_boards(data: &str) -> Vec<Board> {
    data.split("\n\n")
        .map(|x| Board::new(x))
        .collect::<Vec<Board>>()
}


fn data_parser() -> (Vec<u32>, Vec<Board>) {
    let numbers = vec![
        1,76,38,96,62,41,27,33,4,2,94,15,89,25,66,14,30,0,71,21,48,44,87,73,60,50,77,45,29,
        18,5,99,65,16,93,95,37,3,52,32,46,80,98,63,92,24,35,55,12,81,51,17,70,78,61,91,54,8,
        72,40,74,68,75,67,39,64,10,53,9,31,6,7,47,42,90,20,19,36,22,43,58,28,79,86,57,49,83,
        84,97,11,85,26,69,23,59,82,88,34,56,13
    ];

    let data = load_data("data/aoc2021/d04.txt");
    let boards = gen_boards(&data);

    (numbers, boards)
}


fn call_numbers(numbers: Vec<u32>, mut boards: Vec<Board>) -> Option<(u32, Board)> {
    for i in numbers {
        for board in boards.iter_mut() {
            if board.is_bingo(i) {
                return Some((i, board.clone()))
            }
        }
    }

    None
}

fn pick_last_winner(numbers: Vec<u32>, mut boards: Vec<Board>) -> Option<(u32, Board)> {
    let mut winners: Vec<usize> = Vec::new();
    let length = boards.len();

    for num in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if winners.contains(&i) {continue;}

            if board.is_bingo(num) {
                winners.push(i);
                if winners.len() == length {
                    return Some((num, board.clone()))
                }
            }
        }
    }
    
    None
}


fn p1() {
    let (numbers, boards) = data_parser();
    if let Some((num, board)) = call_numbers(numbers, boards) {
        let res = num * board.count();
        println!("{}", res);
    }
}


fn p2() {
    let (numbers, boards) = data_parser();
    if let Some((num, board)) = pick_last_winner(numbers, boards) {
        let res = num * board.count();
        println!("{}", res);
    }
}


pub fn run() {
    // p1();
    p2();
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_number() {
        let data = "22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

        let numbers = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

        let boards = gen_boards(data);
        let res = call_numbers(numbers, boards).unwrap();
        let count = res.1.count();

        assert_eq!((24, 188), (res.0, count))
    }

    #[test]
    fn pick_last() {
        let data = "22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7";

        let numbers = vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1];

        let boards = gen_boards(data);
        println!("{:?}", boards);
        let res = pick_last_winner(numbers, boards).unwrap();
        let count = res.1.count();

        assert_eq!((13, 148), (res.0, count))
    }
}