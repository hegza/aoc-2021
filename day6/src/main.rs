use fs_err as fs;
use itertools::Itertools;
use std::collections::HashMap;

fn windows_mut_each<T>(v: &mut [T], n: usize, mut f: impl FnMut(&mut [T])) {
    let mut start = 0;
    let mut end = n;
    while end <= v.len() {
        f(&mut v[start..end]);
        start += 1;
        end += 1;
    }
}
fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("input.txt").lines();
    let timers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|l| l.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()?;
    //let timers = vec![3, 4, 3, 1, 2];

    let mut buckets = vec![0; 9];
    for t in timers {
        buckets[t as usize] += 1;
    }

    //println!("{:?}", buckets);
    for _ in 0..256 {
        let spawn = buckets[0];

        for i in 0..8 {
            buckets[i] = buckets[i + 1];
        }
        buckets[6] += spawn;

        buckets[8] = spawn;
        //println!("{:?}", buckets);
    }

    let answer: u64 = buckets.into_iter().sum();

    println!("{}", answer);
    Ok(())
}
