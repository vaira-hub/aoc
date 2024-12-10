use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::result;

#[derive(Debug, Hash, Eq, PartialEq, Default)]
struct Points {
    x: u8,
    y: u8,
}

impl Points {
    fn print(&self) {
        println!("Pts are: ({}, {})", self.x, self.y)
    }
    fn top(&self) {
        println!("Pts are: ({}, {})", self.x, self.y)
    }
}

fn read_points(filename: &str) -> io::Result<(Vec<Points>, Points)> {
    let mut pt = Vec::new();
    let mut curr_pt= Points::default();

    // Open the file
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    // Read each line
    for (x, line) in reader.lines().enumerate() {
        let line = line?;
        let x: u8 = TryInto::<u8>::try_into(x).unwrap() + 1;

        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                let y: u8 = TryInto::<u8>::try_into(y).unwrap() + 1;
                let point = Points { x, y };
                pt.push(point);
            }
            if c == '^'{
                let y: u8 = TryInto::<u8>::try_into(y).unwrap() + 1;
                curr_pt = Points { x, y };
            }
        }
    }

    Ok((pt, curr_pt))
}

fn main() -> io::Result<()> {
    let (pts_v, curr_pt) = read_points("day6.txt").unwrap();

    curr_pt.print();
    for p in &pts_v {
        p.print();
    }
    
    let mut pts_visited = HashSet::new();
    let mut dir: (i8, i8) = (1,1);
    loop {
        pts_visited.insert(curr_pt);
        break;
    }

    // for p in pts_visited {
    //     p.print();
    // }

    Ok(())
}
