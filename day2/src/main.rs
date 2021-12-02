fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();
    let numbers = lines
        .map(|l| {
            let mut it = l.split_ascii_whitespace();
            (
                it.next().unwrap().to_owned(),
                it.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(String, i32)>>();

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (instr, am) in numbers {
        match instr.as_ref() {
            "forward" => {
                pos += am;
                depth += aim * am;
            }
            "down" => {
                aim += am;
            }
            "up" => {
                aim -= am;
            }
            _ => panic!(),
        }
    }

    let answer = pos * depth;

    println!("{}", answer);
    Ok(())
}
