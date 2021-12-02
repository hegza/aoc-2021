use fs_err as fs;

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();
    let depth = lines
        .map(|l| l.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()?;

    // Code here
    let deltas = depth.windows(3).map(|d| d[0] + d[1] + d[2]);

    let answer = deltas
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|d| d[1] > d[0])
        .count();

    println!("{}", answer);
    Ok(())
}
