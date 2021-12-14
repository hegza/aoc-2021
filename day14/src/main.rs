use std::collections::HashMap;

use itertools::{self, Itertools};

type RulesMap<'a> = HashMap<&'a str, char>;

fn step(template: &str, pairs: &RulesMap) -> String {
    let mut out = String::with_capacity(template.len() * 2 - 1);

    for (l, r) in template.chars().tuple_windows() {
        out.push(l);

        let key: String = [l, r].iter().collect();
        if let Some(x) = pairs.get(&key.as_str()) {
            out.push(*x);
        }
    }

    out.push(template.chars().nth(template.len() - 1).unwrap());

    out
}

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("input.txt").lines();

    let template = lines.next().unwrap();

    // Skip empty line
    lines.next().unwrap();

    let mut rules = RulesMap::new();
    while let Some(line) = lines.next() {
        let (rule, out) = line.split_once(" -> ").unwrap();
        rules.insert(rule, out.parse::<char>().unwrap());
    }

    let mut template = String::from(template);
    for i in 0..10 {
        println!("Iter: {}", i);
        template = step(&template, &rules);
    }

    let mut counts = HashMap::<char, usize>::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mc = counts
        .iter()
        .max_by(|&(_c1, cnt1), &(_c2, cnt2)| cnt1.cmp(cnt2))
        .unwrap();
    let lc = counts
        .iter()
        .min_by(|&(_c1, cnt1), &(_c2, cnt2)| cnt1.cmp(cnt2))
        .unwrap();

    println!("mc: {}, cnt: {}", mc.0, mc.1);
    println!("lc: {}, cnt: {}", lc.0, lc.1);

    println!("{}", mc.1 - lc.1);
    Ok(())
}
