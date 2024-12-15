use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn day6() {
    let (pts_v, mut curr_pt, max_len) = read_points("day6.txt").unwrap();

    let mut pts_visited = HashSet::new();
    loop {
        let _top_end: &Points;
        match curr_pt.find_top(&pts_v) {
            Some(value) => {
                _top_end = value;
            }
            None => {
                guards_visited(
                    &Points {
                        x: max_len as u8,
                        y: curr_pt.y,
                    },
                    &curr_pt,
                    &mut pts_visited,
                );
                break;
            }
        }
        let top_end = Points {
            x: _top_end.x + 1,
            y: _top_end.y,
        };
        guards_visited(&top_end, &curr_pt, &mut pts_visited);
        let _right_end: &Points;
        match top_end.find_right(&pts_v) {
            Some(value) => {
                _right_end = value;
            }
            None => {
                guards_visited(
                    &top_end,
                    &Points {
                        x: top_end.x,
                        y: max_len as u8,
                    },
                    &mut pts_visited,
                );
                break;
            }
        }
        let right_end = Points {
            x: _right_end.x,
            y: _right_end.y - 1,
        };
        guards_visited(&top_end, &right_end, &mut pts_visited);
        let _bottom_end: &Points;
        match right_end.find_bottom(&pts_v) {
            Some(value) => {
                _bottom_end = value;
            }
            None => {
                guards_visited(
                    &right_end,
                    &Points {
                        x: max_len as u8,
                        y: right_end.y,
                    },
                    &mut pts_visited,
                );
                break;
            }
        }
        let bottom_end = Points {
            x: _bottom_end.x - 1,
            y: _bottom_end.y,
        };
        guards_visited(&right_end, &bottom_end, &mut pts_visited);
        let _left_end: &Points;
        match bottom_end.find_left(&pts_v) {
            Some(value) => {
                _left_end = value;
            }
            None => {
                guards_visited(
                    &Points {
                        x: bottom_end.x,
                        y: max_len as u8,
                    },
                    &bottom_end,
                    &mut pts_visited,
                );
                break;
            }
        }
        let left_end = Points {
            x: _left_end.x,
            y: _left_end.y + 1,
        };
        guards_visited(&left_end, &bottom_end, &mut pts_visited);
        curr_pt = left_end;
    }

    println!("Day6 Part 1: {}", pts_visited.len());
}

fn guards_visited(x_pt: &Points, y_pt: &Points, pts_visited: &mut HashSet<Points>) {
    for point in point_range(x_pt.clone(), y_pt.clone()) {
        pts_visited.insert(point);
    }
}

fn read_points(filename: &str) -> io::Result<(Vec<Points>, Points, usize)> {
    let mut pt = Vec::new();
    let mut curr_pt = Points::default();
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
            if c == '^' {
                let y: u8 = TryInto::<u8>::try_into(y).unwrap() + 1;
                curr_pt = Points { x, y };
            }
        }
    }

    Ok((pt, curr_pt, max_len))
}

#[derive(Debug, Hash, Eq, PartialEq, Default, Clone)]
struct Points {
    pub x: u8,
    pub y: u8,
}

impl Points {
    pub fn find_top<'a>(&self, pts_v: &'a Vec<Points>) -> Option<&'a Points> {
        pts_v
            .iter()
            .filter(|point| point.x < self.x && point.y == self.y)
            .max_by_key(|point| point.x)
            .map(|point| point)
    }
    pub fn find_right<'a>(&self, pts_v: &'a Vec<Points>) -> Option<&'a Points> {
        pts_v
            .iter()
            .filter(|point| point.y > self.y && point.x == self.x)
            .min_by_key(|point| point.y)
            .map(|point| point)
    }
    pub fn find_bottom<'a>(&self, pts_v: &'a Vec<Points>) -> Option<&'a Points> {
        pts_v
            .iter()
            .filter(|point| point.x > self.x && point.y == self.y)
            .min_by_key(|point| point.x)
            .map(|point| point)
    }
    pub fn find_left<'a>(&self, pts_v: &'a Vec<Points>) -> Option<&'a Points> {
        pts_v
            .iter()
            .filter(|point| point.y < self.y && point.x == self.x)
            .max_by_key(|point| point.y)
            .map(|point| point)
    }
}

fn point_range(start: Points, end: Points) -> PointRange {
    PointRange::new(start, end)
}

#[derive(Clone)]
struct PointRange {
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
