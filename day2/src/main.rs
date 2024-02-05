fn main() {
    let input = include_str!("input");
    let result:u32 = input.lines().map(action).sum();
    println!("{result}");
}

fn action(input: &str) -> u32{
    match input {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!()
        }

}