use itertools::{self, Itertools};

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();
    let numbers = lines
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    // Code here
    let answer = 0;

    println!("{}", answer);
    Ok(())
}
