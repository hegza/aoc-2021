use std::collections::HashMap;

use itertools::{self, Itertools};

type RulesMap<'a> = HashMap<&'a str, char>;
type CharToCount = HashMap<char, usize>;

fn counts_memoized(template: &str, step_count: usize, rules: &RulesMap) -> CharToCount {
    let mut counts = HashMap::new();
    for c in template.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut memo = HashMap::new();
    for (l, r) in template.chars().tuple_windows() {
        let new_counts = insert_and_memoize(l, r, step_count, rules, &mut memo);
        for (c, n) in new_counts {
            *counts.entry(c).or_insert(0) += n;
        }
    }
    counts
}

type Params = (char, char, usize);

fn insert_and_memoize(
    l: char,
    r: char,
    step_count: usize,
    rules: &RulesMap,
    memo: &mut HashMap<Params, CharToCount>,
) -> CharToCount {
    if let Some(counts) = memo.get(&(l, r, step_count)) {
        counts.clone()
    } else {
        let mut char_to_count = CharToCount::new();
        if let Some(&x) = rules.get(&[l, r].iter().collect::<String>().as_str()) {
            char_to_count.insert(x, 1);
            if step_count > 1 {
                for (c, n) in insert_and_memoize(l, x, step_count - 1, rules, memo)
                    .iter()
                    .chain(insert_and_memoize(x, r, step_count - 1, rules, memo).iter())
                {
                    *char_to_count.entry(*c).or_insert(0) += n;
                }
            }
        }
        memo.insert((l, r, step_count), char_to_count.clone());
        char_to_count
    }
}

fn insert(l: char, r: char, out: &mut String, pairs: &RulesMap) {
    out.push(l);

    let key: String = [l, r].iter().collect();
    if let Some(x) = pairs.get(&key.as_str()) {
        out.push(*x);
    }
}

fn step(template: &str, pairs: &RulesMap) -> String {
    let mut out = String::with_capacity(template.len() * 2 - 1);

    for (l, r) in template.chars().tuple_windows() {
        insert(l, r, &mut out, pairs);
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

    let counts = counts_memoized(&template, 40, &rules);

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
