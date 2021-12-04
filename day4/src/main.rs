use fs_err as fs;
use itertools::Itertools;

#[derive(Debug)]
struct Board(Vec<Vec<u8>>);

#[derive(Debug, Clone)]
struct ConvBoard(Vec<Vec<u8>>);

impl ConvBoard {
    fn from_board(b: &Board) -> ConvBoard {
        let mut all =
        // Verticals
        (0..5_usize).into_iter().map(|idx| {
            b.0.iter().map(|hor| hor[idx]).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        all.append(&mut b.0.clone());

        ConvBoard(all)
    }
}

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("input.txt").lines();

    let order_line = lines.next().unwrap();
    let order = order_line
        .split(',')
        .map(|e| e.parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()?;
    println!("Order: {:?}", &order);

    // Skip one
    lines.next().unwrap();
    //let lines = lines.skip(1);

    //
    let mut boards = vec![];
    for chunk in &lines.chunks(6) {
        // The first 5 are a board
        let arr = chunk
            .take(5)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|e| e.parse::<u8>())
                    .collect::<Result<Vec<u8>, _>>()
                    .unwrap()
            })
            .collect::<Vec<Vec<u8>>>();

        let board = Board(arr);
        boards.push(board);
        // Skip 6th
    }

    let mut conv_boards: Vec<ConvBoard> = boards.iter().map(|b| ConvBoard::from_board(b)).collect();
    println!("ConvBoards");
    for b in &conv_boards {
        println!("{:?}", b);
    }

    let (b, drawn) = find_last_bingo(&mut conv_boards, &order);

    let score =
        b.0.iter()
            .take(5)
            .flatten()
            .filter(|n| !drawn.contains(n))
            .map(|n| *n as u64)
            .sum::<u64>()
            * (*drawn.last().unwrap() as u64);

    println!("{}", score);
    Ok(())
}

fn find_bingo<'a, 'b>(conv_boards: &'a [ConvBoard], order: &'b [u8]) -> (&'a ConvBoard, Vec<u8>) {
    let mut drawn = Vec::with_capacity(order.len());
    for draw in order {
        drawn.push(*draw);

        for b in conv_boards {
            let mut lines = b.0.iter();
            let is_bingo = lines.any(|line| line.iter().all(|n| drawn.contains(n)));
            if is_bingo {
                println!("Bingo board: {:?}", b);
                return (b, drawn);
            }
        }
    }
    panic!()
}

fn find_last_bingo<'a, 'b>(
    conv_boards: &'a mut Vec<ConvBoard>,
    order: &'b [u8],
) -> (ConvBoard, Vec<u8>) {
    let mut drawn = Vec::with_capacity(order.len());
    for draw in order {
        drawn.push(*draw);

        let mut rem_indices = vec![];
        for (idx, b) in conv_boards.iter().enumerate() {
            let mut lines = b.0.iter();
            let is_bingo = lines.any(|line| line.iter().all(|n| drawn.contains(n)));
            if is_bingo {
                rem_indices.push(idx);
                println!("Board {} won", idx);

                if conv_boards.len() - rem_indices.len() == 0 {
                    println!("Board {:?} is the last to win", &b);
                    return (b.clone(), drawn);
                }
            }
        }
        for ri in rem_indices.iter().rev() {
            conv_boards.swap_remove(*ri);
        }
    }
    panic!()
}
