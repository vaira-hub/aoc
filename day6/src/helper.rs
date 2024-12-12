use std::fs::File;
use std::io::{self, BufRead};

pub fn read_points(filename: &str) -> io::Result<(Vec<Points>, Points, usize)> {
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

#[derive(Debug, Hash, Eq, PartialEq, Default, Clone)]
pub struct Points {
    pub x: u8,
    pub y: u8,
}

impl Points {
    pub fn find_top<'a>(&self, pts_v:&'a Vec<Points>) -> Option<&'a Points> {
        pts_v.iter().filter(|point| point.x < self.x && point.y == self.y)
        .max_by_key(|point| point.x)
        .map(|point| point)
    }
    pub fn find_right<'a>(&self, pts_v:&'a Vec<Points>) -> Option<&'a Points>{
        pts_v.iter().filter(|point| point.y > self.y && point.x == self.x)
        .min_by_key(|point| point.y)
        .map(|point| point)
    }
    pub fn find_bottom<'a>(&self, pts_v:&'a Vec<Points>) -> Option<&'a Points>{
        pts_v.iter().filter(|point| point.x > self.x && point.y == self.y)
        .min_by_key(|point| point.x)
        .map(|point| point)
    }
    pub fn find_left<'a>(&self, pts_v:&'a Vec<Points>) -> Option<&'a Points>{
        pts_v.iter().filter(|point| point.y < self.y && point.x == self.x)
        .max_by_key(|point| point.y)
        .map(|point| point)
    }
}

pub fn point_range(start: Points, end: Points) -> PointRange {
    PointRange::new(start, end)
}

#[derive(Clone)]
pub struct PointRange {
    end: Points,
    current: Points,
}

impl PointRange {
    fn new(start: Points, end: Points) -> Self {
        PointRange {
            end,
            current: start.clone(),
        }
    }
}

impl Iterator for PointRange {
    type Item = Points;

    fn next(&mut self) -> Option<Self::Item> {
        // End itertator
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