fn main() {
    let input = include_str!("input");
    let result:u32 = input.lines().map(action).sum();
    println!("{result}");

    let input = include_str!("input");
    let result:u32 = input.lines().map(actionresult).sum();
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
fn actionresult(input: &str) -> u32{
    match input {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => panic!()
        }

}