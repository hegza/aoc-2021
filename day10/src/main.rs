use anyhow::Context;
use itertools::{self, Itertools};
use maplit::hashmap;
use std::collections::{HashMap, VecDeque};

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();
    let errs = lines.enumerate().filter_map(|(idx, l)| {
        let res = parse(l);
        match &res {
            Ok(_) => {}
            Err(e) => println!("Parse error on line: {} at {}", idx + 1, e.loc + 1),
        }
        res.err()
    }); //.collect::<Vec<ParseError>>();

    let score: u32 = errs
        .map(|e| match e.kind {
            ParenKind::Paren => 3,
            ParenKind::Square => 57,
            ParenKind::Brace => 1197,
            ParenKind::Angle => 25137,
            ParenKind::Unknown => panic!("{:?}", e.kind),
        })
        .sum();

    println!("{}", score);
    Ok(())
}

struct ParseError {
    kind: ParenKind,
    loc: usize,
}

#[derive(Debug)]
enum ParenKind {
    // '('
    Paren,
    // '['
    Square,
    // '{'
    Brace,
    // '<'
    Angle,
    Unknown,
}

fn parse(line: &str) -> Result<(), ParseError> {
    let mut chars = line.chars().enumerate();

    let mut open_stack = VecDeque::new();

    while let Some((loc, c)) = chars.next() {
        match c {
            '(' => {
                open_stack.push_back('(');
            }
            ')' => {
                if let Some(back) = open_stack.pop_back() {
                    if back != '(' {
                        return Err(ParseError {
                            kind: ParenKind::Paren,
                            loc,
                        });
                    }
                }
            }
            '[' => {
                open_stack.push_back('[');
            }
            ']' => {
                if let Some(back) = open_stack.pop_back() {
                    if back != '[' {
                        return Err(ParseError {
                            kind: ParenKind::Square,
                            loc,
                        });
                    }
                }
            }
            '{' => {
                open_stack.push_back('{');
            }
            '}' => {
                if let Some(back) = open_stack.pop_back() {
                    if back != '{' {
                        return Err(ParseError {
                            kind: ParenKind::Brace,
                            loc,
                        });
                    }
                }
            }
            '<' => {
                open_stack.push_back('<');
            }
            '>' => {
                if let Some(back) = open_stack.pop_back() {
                    if back != '<' {
                        return Err(ParseError {
                            kind: ParenKind::Angle,
                            loc,
                        });
                    }
                }
            }
            _ => {
                return Err(ParseError {
                    kind: ParenKind::Unknown,
                    loc,
                })
            }
        }
    }

    Ok(())
}
