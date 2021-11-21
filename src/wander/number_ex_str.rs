

fn unique_numbers(text: String) -> usize {
    let mut res = text
        .chars()
        .map(|x| {if x >= '0' && x <= '9' {x} else {' '}})
        .collect::<String>()
        .split_whitespace()
        .map(|x| x.parse::<i32>()
        .unwrap()).collect::<Vec<i32>>();

    res.sort();
    res.dedup();
    res.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let text = String::from("a123bc34d8ef34");
        assert_eq!(3, unique_numbers(text));
    }

    #[test]
    fn case2() {
        let text = String::from("leet1234code234");
        assert_eq!(2, unique_numbers(text));
    }

    #[test]
    fn case3() {
        let text = String::from("a1b01c001");
        assert_eq!(1, unique_numbers(text));
    }
}