fn fold(map: &Vec<Vec<char>>, axis: char, n: usize) -> Vec<Vec<char>> {
    match axis {
        'y' => {
            let mut out = map[0..=n - 1].to_vec();
            for (idx, line) in map[n + 1..].iter().enumerate() {
                let y = n - 1 - idx;
                for (x, c) in line.iter().enumerate() {
                    if c == &'#' && out[y][x] == '.' {
                        out[y][x] = '#';
                    }
                }
            }
            out
        }
        'x' => {
            let mut out = vec![vec!['.'; n]; map.len()];

            for (idx, line) in map.iter().enumerate() {
                out[idx] = line[0..=n - 1].to_vec();
            }

            for (y, line) in map.iter().enumerate() {
                for (idx, c) in line[n + 1..].iter().enumerate() {
                    let x = n - 1 - idx;
                    if c == &'#' && out[y][x] == '.' {
                        out[y][x] = '#';
                    }
                }
            }

            out
        }
        _ => panic!(),
    }
}

fn count_dots(map: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for line in map {
        for c in line {
            if c == &'#' {
                count += 1;
            }
        }
    }
    count
}

fn main() -> anyhow::Result<()> {
    let mut lines = include_str!("input.txt").lines();
    let mut dots = Vec::with_capacity(877);

    // Parse dots
    loop {
        let s = lines.next().unwrap();
        if s == "" {
            break;
        }

        let (l, r) = s.split_once(',').unwrap();
        dots.push((l.parse::<usize>()?, r.parse::<usize>()?));
    }

    // Parse folds
    let mut folds = Vec::with_capacity(12);
    while let Some(s) = lines.next() {
        let mut s = s.split_ascii_whitespace();

        // Skip fold
        s.next().unwrap();
        // Skip along
        s.next().unwrap();

        let (axis, n) = s.next().unwrap().split_once('=').unwrap();
        let axis = axis.parse::<char>()?;
        let n = n.parse::<usize>()?;
        folds.push((axis, n));
    }

    let hmax = dots.iter().map(|co| co.0).max().unwrap();
    let vmax = dots.iter().map(|co| co.1).max().unwrap();

    let mut map = vec![vec!['.'; hmax + 1]; vmax + 1];

    for dot in dots {
        map[dot.1][dot.0] = '#';
    }

    for f in folds {
        map = fold(&map, f.0, f.1);
    }
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!()
    }

    Ok(())
}
