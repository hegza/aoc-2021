use std::{cell::RefCell, iter, rc::Rc};

use itertools::{self, izip, Itertools};

struct Square {
    height: u8,
    w: Option<Rc<RefCell<Square>>>,
    e: Option<Rc<RefCell<Square>>>,
    n: Option<Rc<RefCell<Square>>>,
    s: Option<Rc<RefCell<Square>>>,
}

impl Square {
    fn from_height(height: u8) -> Square {
        Square {
            height,
            w: None,
            e: None,
            n: None,
            s: None,
        }
    }
    //fn from_slice
    fn is_local_minimum(&self) -> bool {
        let non_center = &[self.n, self.w, self.e, self.s];

        non_center.iter().all(|&v| v > self.height)
    }
}

struct Board(Vec<Vec<Square>>);

impl Board {
    /*
    fn from_u8(map: &[&[u8]]) -> Board {
        //let mut lines = Vec::with_capacity(map[0].len());

        let lines: Vec<Vec<Square>> = map
            .iter()
            .map(|line| {
                line.iter()
                    .map(|&height| Square::partial(height))
                    .collect_vec()
            })
            .collect();

        Board(lines)
    }*/

    fn from_padded(map: Vec<Vec<u8>>) -> Board {
        Board(
            map.windows(3)
                .map(|three_lines| {
                    let top = &three_lines[0];
                    let vmid = &three_lines[1];
                    let bot = &three_lines[2];

                    izip!(top, vmid, bot)
                        .tuple_windows::<(_, _, _)>()
                        .map(|(left, hmid, right)| {
                            let n = *hmid.0;
                            let w = *left.1;
                            let c = *hmid.1;
                            let e = *right.1;
                            let s = *hmid.2;

                            Square {
                                height: c,
                                w,
                                e,
                                n,
                                s,
                            }
                        })
                        .collect_vec()
                })
                .collect::<Vec<Vec<Square>>>(),
        )
    }
}

fn pad_line(len: usize) -> Vec<u8> {
    vec![9; len]
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();
    let lines: Vec<Vec<u8>> = lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    for line in &lines[0..10] {
        println!("{:?}", &line[0..10]);
    }

    //let board = Board::from_u8(&values);
    let width = lines[0].len();

    let padded = iter::once(pad_line(width))
        .chain(lines.into_iter().map(|line| {
            iter::once(9)
                .chain(line.into_iter())
                .chain(iter::once(9))
                .collect_vec()
        }))
        .chain(iter::once(pad_line(width)))
        .collect::<Vec<Vec<u8>>>();

    let board = Board::from_padded(padded);

    /*
    let mut minima = vec![];
    padded.windows(3).for_each(|three_lines| {
        let top = &three_lines[0];
        let vmid = &three_lines[1];
        let bot = &three_lines[2];

        izip!(top, vmid, bot)
            .tuple_windows::<(_, _, _)>()
            .for_each(|(left, hmid, right)| {
                //let nw = left.0;
                let n = hmid.0;
                //let ne = right.0;
                let w = left.1;
                let c = hmid.1;
                let e = right.1;
                //let sw = left.2;
                let s = hmid.2;
                //let se = right.2;

                /*
                println!("---");
                println!("{:>2} {:>2} {:>2}", nw, n, ne);
                println!("{:>2} {:>2} {:>2}", w, c, e);
                println!("{:>2} {:>2} {:>2}", sw, s, se);
                println!("---");
                */

                let non_center = &[n, w, e, s];
                // Local minimum
                if non_center.iter().all(|&v| v > c) {
                    minima.push(c);
                }
            });
    });
    */
    let minima = board.0.iter().flatten().filter(|&sq| sq.is_local_minimum());

    // Code here
    let answer: u32 = minima.map(|v| v.height as u32 + 1).sum();

    println!("{}", answer);
    Ok(())
}
