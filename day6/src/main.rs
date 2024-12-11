use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Hash, Eq, PartialEq, Default, Clone)]
struct Points {
    x: u8,
    y: u8,
}

impl Points {
    fn print(&self, txt: &str) {
        println!("Pts are {}: ({}, {})", txt, self.x, self.y)
    }
    fn find_top<'a>(&self, pts_v:&'a Vec<Points>) -> Option<&'a Points> {
        pts_v.iter().find(|point| point.x < self.x && point.y == self.y)
    }
    fn find_right<'a>(&self, pts_v:&'a Vec<Points>) -> Option<&'a Points>{
        pts_v.iter().find(|point| point.y > self.y && point.x == self.x )
    }
    fn find_bottom<'a>(&self, pts_v:&'a Vec<Points>) -> Option<&'a Points>{
        pts_v.iter().find(|point| point.x > self.x && point.y == self.y )
    }
    fn find_left<'a>(&self, pts_v:&'a Vec<Points>) -> Option<&'a Points>{
        pts_v.iter().find(|point| point.y < self.y && point.x == self.x )
    }
}

fn read_points(filename: &str) -> io::Result<(Vec<Points>, Points, usize)> {
    let mut pt = Vec::new();
    let mut curr_pt= Points::default();
    let mut max_len: usize = 0;

    // Open the file
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    // Read each line
    for (x, line) in reader.lines().enumerate() {
        let line = line?;
        max_len = line.len();
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

    Ok((pt, curr_pt, max_len))
}

fn main() -> io::Result<()> {
    let (pts_v, mut curr_pt, max_len) = read_points("day6.txt").unwrap();

    let mut pts_visited = HashSet::new();
    let mut dir: (i8, i8) = (1,1);
    loop {
        curr_pt.print("curr");
        let _top_end: &Points;
        match curr_pt.find_top(&pts_v) {
            Some(value) => {
                _top_end = value;
            },
            None => {
                for point in point_range(Points{x:max_len as u8, y:curr_pt.y}, curr_pt) {
                    // point.print("TOP END PTS");
                    pts_visited.insert(point);
                };
                break;
            },
        }
        let top_end = Points {
            x: _top_end.x + 1,
            y: _top_end.y,
        };
        top_end.print("top end");
        for point in point_range(top_end.clone(), curr_pt) {
            // point.print("TOP END PTS");
            pts_visited.insert(point);
        };
        let _right_end: &Points;
        match top_end.find_right(&pts_v) {
            Some(value) => {
                _right_end = value;
            },
            None => {
                for point in point_range(top_end.clone(), Points{x:top_end.x, y:max_len as u8}) {
                    // point.print("RIGHT END PTS");
                    pts_visited.insert(point);
                };
                break;
            },
        }
        let right_end = Points {
            x: _right_end.x,
            y: _right_end.y - 1,
        };
        right_end.print("right end");
        for point in point_range(top_end.clone(), right_end.clone()) {
            // point.print("RIGHT END PTS");
            pts_visited.insert(point);
        }        
        let _bottom_end: &Points;
        match right_end.find_bottom(&pts_v) {
            Some(value) => {
                _bottom_end = value;
            },
            None => {
                for point in point_range(right_end.clone(), Points{x:max_len as u8, y:right_end.y}) {
                    // point.print("BOTTOM END PTS");
                    pts_visited.insert(point);
                };
                break;
            },
        }
        let bottom_end = Points {
            x: _bottom_end.x - 1,
            y: _bottom_end.y,
        };
        bottom_end.print("bot end");
        for point in point_range(right_end.clone(), bottom_end.clone()) {
            // point.print("BOTTOM END PTS");
            pts_visited.insert(point);
        }
        let _left_end: &Points;
        match bottom_end.find_left(&pts_v) {
            Some(value) => {
                _left_end = value;
            },
            None => {
                for point in point_range(Points{x:bottom_end.x, y:max_len as u8}, bottom_end.clone()) {
                    // point.print("LEFT END PTS");
                    pts_visited.insert(point);
                };
                break;
            },
        }
        let left_end = Points {
            x: _left_end.x,
            y: _left_end.y + 1,
        };
        // left_end.print("left end");
        for point in point_range(left_end.clone(), bottom_end.clone()) {
            // point.print("LEFT END PTS");
            pts_visited.insert(point);
        }
        curr_pt = left_end;
        // break;
    }

    // Add points without boundary

    // for p in pts_visited {
    //     p.print("orig v");
    // }
    println!("{}",pts_visited.len());
    Ok(())
}

fn point_range(start: Points, end: Points) -> PointRange {
    PointRange::new(start, end)
}

#[derive(Clone)]
struct PointRange {
    start: Points,
    end: Points,
    current: Points,
}

impl PointRange {
    fn new(start: Points, end: Points) -> Self {
        PointRange {
            start: start.clone(),
            end,
            current: start.clone(),
        }
    }
}

impl Iterator for PointRange {
    type Item = Points;

    fn next(&mut self) -> Option<Self::Item> {
        // End loop
        // println!("it: {}, {}, {}, {}", self.current.x, self.end.x, self.current.y, self.end.y);
        if self.current.x > self.end.x && self.current.y > self.end.y {
            return None;
        }

        let point = self.current.clone();

        if self.current.x == self.end.x && self.current.y == self.end.y {
            self.current.x += 1;
            self.current.y += 1;
        }

        if self.current.x < self.end.x {
            self.current.x += 1;
        } else if self.current.y < self.end.y {
            self.current.y += 1;
        }

        Some(point)
    }
}