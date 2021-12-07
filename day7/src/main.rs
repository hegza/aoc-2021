use statrs::statistics::Statistics;

fn ari_sum(n: u64) -> u64 {
    (n as f64 * ((1 + n) as f64 / 2.)) as u64
}

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("input.txt").lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<u64>().unwrap());

    // Use median for Part One
    let mean = numbers
        .clone()
        .map(|i| i as f64)
        .collect::<Vec<f64>>()
        .mean();
    let int_mean = mean.round() as i64;

    let numbers = numbers.collect::<Vec<u64>>();

    let f = fuel(&numbers, int_mean);

    println!("mean: {} -> {}", mean, int_mean);

    println!("{}: {:?}", int_mean, f);
    // Somehow, mean gave me an off-by-one. The correct solution was found at 476
    println!("{}: {:?}", 476, fuel(&numbers, 476));
    println!("{}: {:?}", 475, fuel(&numbers, 475));
    Ok(())
}

fn fuel(numbers: &[u64], mean: i64) -> u64 {
    let mut fuel: u64 = 0;
    for n in numbers {
        let diff = (*n as i64 - mean).abs() as u64;
        let sum = ari_sum(diff);
        //println!("sum: {} -> {}: {}", n, int_mean, sum);
        fuel += sum as u64;
    }
    fuel
}
