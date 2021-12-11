use itertools::{self, izip, Itertools};

#[derive(Clone, PartialEq)]
enum Energy {
    Level(u8),
    Flashing,
}

struct Board {
    vals: Vec<Vec<Energy>>,
}

impl Board {
    fn from_levels(levels: Vec<Vec<u8>>) -> Board {
        let vals = levels
            .into_iter()
            .map(|line| line.into_iter().map(|v| Energy::Level(v)).collect_vec())
            .collect_vec();
        Board { vals }
    }
    fn increase_energy_in_area(&mut self, x0: usize, x1: usize, y0: usize, y1: usize) -> usize {
        let mut flashed = vec![];
        for (y, line) in self.vals[y0..=y1].iter_mut().enumerate() {
            for (x, elem) in line[x0..=x1].iter_mut().enumerate() {
                match elem {
                    Energy::Level(l) => {
                        if *l + 1 > 9 {
                            *elem = Energy::Flashing;
                            flashed.push((x + x0, y + y0));
                            //println!("({}, {}) will flash", x, y);
                        } else {
                            //println!("({}, {})", x, y);
                            *l += 1;
                        }
                    }
                    Energy::Flashing => {}
                }
            }
        }
        //println!();

        let mut flash_count = flashed.len();

        for (x, y) in flashed {
            let x0 = (x as isize - 1).max(0) as usize;
            let y0 = (y as isize - 1).max(0) as usize;
            let x1 = (x + 1).min(self.vals[0].len() - 1);
            let y1 = (y + 1).min(self.vals.len() - 1);
            //println!("flashing ({}, {}) {} {} {} {}", x, y, x0, x1, y0, y1);
            flash_count += self.increase_energy_in_area(x0, x1, y0, y1);
        }
        flash_count
    }
    fn fix_levels(&mut self) -> bool {
        let mut all_flashed = true;
        for line in &mut self.vals {
            for elem in line {
                if *elem == Energy::Flashing {
                    *elem = Energy::Level(0);
                } else {
                    all_flashed = false;
                }
            }
        }
        all_flashed
    }
    fn step(&mut self) -> (usize, bool) {
        // Increase all levels by one
        let flash_count =
            self.increase_energy_in_area(0, self.vals[0].len() - 1, 0, self.vals.len() - 1);

        let sync = self.fix_levels();

        (flash_count, sync)
    }
    fn print(&self) {
        for line in &self.vals {
            println!(
                "{:?}",
                &line
                    .iter()
                    .map(|elem| {
                        match elem {
                            Energy::Level(l) => l,
                            Energy::Flashing => panic!(),
                        }
                    })
                    .collect_vec()
            );
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();
    let levels: Vec<Vec<u8>> = lines
        .map(|l| {
            l.chars()
                .into_iter()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
        })
        .collect_vec();

    let mut board = Board::from_levels(levels);

    let mut flash_count = 0;
    for step in 0.. {
        let (fc, sync) = board.step();
        flash_count += fc;
        if sync {
            println!("{}", step + 1);
            break;
        }
        /*board.print();
        println!();
        */
    }

    //println!("{}", flash_count);
    Ok(())
}
