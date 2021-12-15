use std::collections::BinaryHeap;

use itertools::{self, Itertools};

fn dijkstra(start: (usize, usize), finish: (usize, usize), grid: &Vec<Vec<isize>>) -> isize {
    let width = grid[0].len();
    let height = grid.len();

    let mut heap = BinaryHeap::new();
    // Push starting node
    heap.push((0isize, start));

    // Init dist map
    let mut dist = vec![vec![isize::MAX; width]; height];
    dist[start.1][start.0] = 0;

    while let Some((cost, (x, y))) = heap.pop() {
        // Use negative cost because BinaryHeap is a max heap
        let cost = -cost;
        if (x, y) == finish {
            return cost;
        }

        if cost > dist[y][x] as isize {
            continue;
        }

        for (y1, x1) in [(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)] {
            if grid.get(y1).and_then(|row| row.get(x1)).is_none() {
                continue;
            }
            let ncost = cost + grid[y1][x1] as isize;
            if ncost < dist[y1][x1] as isize {
                heap.push((-ncost, (x1, y1)));
                dist[y1][x1] = ncost;
            }
        }
    }
    unreachable!()
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();

    let grid = lines
        .map(|l| {
            l.chars()
                .map(|i| i.to_digit(10).unwrap() as isize)
                .collect_vec()
        })
        .collect_vec();
    let full = (0..5 * grid.len())
        .map(|x| {
            (0..5 * grid[0].len())
                .map(|y| {
                    let inc_from_x = (x / grid.len()) as isize;
                    let inc_from_y = (y / grid[0].len()) as isize;
                    let cost = grid[x % grid.len()][y % grid[0].len()] + inc_from_x + inc_from_y;
                    if cost < 10 {
                        cost
                    } else {
                        cost - 9
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let w = dijkstra((0, 0), (full[0].len() - 1, full.len() - 1), &full);

    println!("{}", w);
    Ok(())
}
