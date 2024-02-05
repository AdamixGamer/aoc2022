fn main() {
    let input = include_str!("input");

    let result = input.split("\n\n").map(number).max().unwrap();
    println!("{result}");

    let input = include_str!("input");

    let mut result:Vec<u64> = input.split("\n\n").map(number).collect();
    result.sort();
    let len = result.len();
    let result: u64 = result[(len-3)..].into_iter().sum();
    println!("{result}");  
}

fn number(string: &str) -> u64 {
    string.lines().map(|x| x.parse::<u64>().unwrap()).sum()
}
