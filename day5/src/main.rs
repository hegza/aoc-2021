use fs_err as fs;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn from_coords(c: (i32, i32)) -> Point {
        Point { x: c.0, y: c.1 }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Line {
    start: Point,
    end: Point,
    /*
    a: i32,
    b: i32,
    */
}

impl Line {
    fn from_coords(c1: (i32, i32), c2: (i32, i32)) -> Line {
        let start = Point::from_coords(c1);
        let end = Point::from_coords(c2);
        /*
        let a = (start.y - end.y) / (start.x - end.y);
        let b = start.y - a * start.x;
        */
        Line { start, end }
    }
    fn is_diag(&self) -> bool {
        (!self.is_hor()) && (!self.is_ver())
    }
    fn is_hor(&self) -> bool {
        self.start.y == self.end.y
    }
    fn is_ver(&self) -> bool {
        self.start.x == self.end.x
    }
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<i32>>);

impl Board {
    fn render(&mut self, line: &Line) {
        if line.is_hor() {
            let y = line.start.y;
            let x0 = line.start.x.min(line.end.x);
            let x1 = line.start.x.max(line.end.x);
            for x in x0..=x1 {
                self.0[y as usize][x as usize] += 1;
            }
        } else if line.is_ver() {
            let y0 = line.start.y.min(line.end.y);
            let y1 = line.start.y.max(line.end.y);
            let x = line.start.x as usize;
            for y in y0..=y1 {
                self.0[y as usize][x as usize] += 1;
            }
        } else {
            let step = (
                if line.end.x >= line.start.x { 1 } else { -1 },
                if line.end.y >= line.start.y { 1 } else { -1 },
            );

            let mut cursor = line.start.clone();
            while cursor != line.end {
                self.0[cursor.y as usize][cursor.x as usize] += 1;
                cursor.x += step.0;
                cursor.y += step.1;
            }
            self.0[cursor.y as usize][cursor.x as usize] += 1;
        }
    }

    fn format_print(&self) {
        for line in &self.0 {
            println!("{}", line.iter().join(" "));
        }
    }
}

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();
    let lines = lines
        .map(|l| {
            let mut it = l.split(" -> ");

            let first_two = it.next().unwrap();
            let mut it1 = first_two.split(',');
            let x1 = it1.next().unwrap().parse::<i32>().unwrap();
            let y1 = it1.next().unwrap().parse::<i32>().unwrap();

            let next_two = it.next().unwrap();
            let mut it2 = next_two.split(',');
            let x2 = it2.next().unwrap().parse::<i32>().unwrap();
            let y2 = it2.next().unwrap().parse::<i32>().unwrap();

            Line::from_coords((x1, y1), (x2, y2))
        })
        .collect::<Vec<Line>>();
    //println!("{:?}", lines);

    //let non_diag_lines = lines.into_iter().filter(|l| !l.is_diag());

    let mut board = Board(vec![vec![0; 999]; 999]);

    for l in lines {
        board.render(&l);
    }

    let mut count = 0;
    for y in board.0 {
        for val in y {
            if val >= 2 {
                count += 1;
            }
        }
    }

    println!("{}", count);
    Ok(())
}
