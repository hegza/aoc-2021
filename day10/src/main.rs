use std::collections::VecDeque;

use itertools::Itertools;

fn value(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!(),
    }
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();

    let incomplete = lines.filter_map(|l| parse(l).ok());

    let mut line_scores = incomplete
        .map(|missing| {
            missing
                .into_iter()
                .rev()
                .fold(0, |score, c| score * 5 + value(c))
        })
        .collect_vec();

    line_scores.sort();

    let mid_score = line_scores[line_scores.len() / 2];
    println!("{}", mid_score);

    Ok(())
}

struct ParseError {
    kind: char,
    loc: usize,
}

fn matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!(),
    }
}

struct ParseStack(VecDeque<char>);

impl ParseStack {
    fn push(&mut self, c: char) {
        self.0.push_back(c);
    }
    fn pop(&mut self, c: char, loc: usize) -> Result<(), ParseError> {
        if let Some(back) = self.0.pop_back() {
            if back != matching(c) {
                return Err(ParseError { kind: '(', loc });
            }
        }
        Ok(())
    }
}

fn parse(line: &str) -> Result<VecDeque<char>, ParseError> {
    let mut chars = line.chars().enumerate();

    let mut parse_stack = ParseStack(VecDeque::new());

    while let Some((loc, c)) = chars.next() {
        match c {
            '(' | '[' | '{' | '<' => parse_stack.push(c),
            ')' | ']' | '}' | '>' => parse_stack.pop(c, loc)?,
            _ => panic!("column {}", loc + 1),
        }
    }

    Ok(parse_stack.0)
}
