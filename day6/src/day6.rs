use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

pub fn day6() {
    let (mut pts_v, top_start, max_len) = read_points("day6.txt").unwrap();
    let start = top_start.clone();

    let mut pts_loop: HashSet<Points> = HashSet::new();
    let mut pts_visited: HashSet<Points> = HashSet::new();
    find_pts(max_len, top_start, &pts_v, &mut pts_visited, &mut pts_loop);

    println!("Day6 Part 1: {}", pts_visited.len());
    let mut pts_loop_f: HashSet<Points> = HashSet::new();
    for p in pts_loop{
        if pts_v.contains(&p){
            continue;
        }
        pts_loop_f.insert(p);
    }
    let mut counter: u16 = 0;
    pts_loop_f.retain(|x| x != &start);
    println!("pts rep {:?}", pts_loop_f.len());
    for p in pts_loop_f.iter() {
        pts_v.push(p.clone());
        let is_loop = check_for_loop(&start, &pts_v);
        if is_loop {
            counter += 1;
        }
        pts_v.retain(|x| x != p);
    }
    println!("Day6 Part 2: {:?}", counter);
}

fn check_for_loop(start: &Points, pts_v: &Vec<Points>) -> bool {
    let mut top_start = start.clone();
    let mut loop_finder: HashSet<Points> = HashSet::new();
    loop {
        let _top_end: &Points;
        match top_start.find_top(&pts_v) {
            Some(value) => {
                _top_end = value;
            }
            None => return false
        }
        let top_end = Points {
            x: _top_end.x + 1,
            y: _top_end.y,
        };
        let _right_end: &Points;
        let right_start = top_end;
        match right_start.find_right(&pts_v) {
            Some(value) => {
                _right_end = value;
            }
            None => return false
        }
        let right_end = Points {
            x: _right_end.x,
            y: _right_end.y - 1,
        };
        let _bottom_end: &Points;
        let bottom_start = right_end;
        match bottom_start.find_bottom(&pts_v) {
            Some(value) => {
                _bottom_end = value;
            }
            None => return false
        }
        let bottom_end = Points {
            x: _bottom_end.x - 1,
            y: _bottom_end.y,
        };
        let _left_end: &Points;
        let left_start = bottom_end;
        match left_start.find_left(&pts_v) {
            Some(value) => {
                _left_end = value;
            }
            None => return false
        }
        let left_end = Points {
            x: _left_end.x,
            y: _left_end.y + 1,
        };
        if loop_finder.contains(&top_start) {
            return true
        }
        loop_finder.insert(top_start.clone());
        top_start = left_end;
    }
}

fn find_pts(
    max_len: usize,
    mut top_start: Points,
    pts_v: &Vec<Points>,
    mut pts_visited: &mut HashSet<Points>,
    mut pts_loop: &mut HashSet<Points>,
) {
    loop {
        let _top_end: &Points;
        match top_start.find_top(&pts_v) {
            Some(value) => {
                _top_end = value;
            }
            None => {
                guards_visited(
                    &Points {
                        x: 1,
                        y: top_start.y,
                    },
                    &top_start,
                    &mut pts_visited,
                    "top",
                    &mut pts_loop,
                    pts_v,
                    max_len
                );
                break;
            }
        }
        let top_end = Points {
            x: _top_end.x + 1,
            y: _top_end.y,
        };
        guards_visited(
            &top_end,
            &top_start,
            &mut pts_visited,
            "top",
            &mut pts_loop,
            pts_v,
            max_len
        );
        let _right_end: &Points;
        let right_start = top_end;
        match right_start.find_right(&pts_v) {
            Some(value) => {
                _right_end = value;
            }
            None => {
                guards_visited(
                    &right_start,
                    &Points {
                        x: right_start.x,
                        y: max_len as u8,
                    },
                    &mut pts_visited,
                    "right",
                    &mut pts_loop,
                    pts_v,
                    max_len
                );
                break;
            }
        }
        let right_end = Points {
            x: _right_end.x,
            y: _right_end.y - 1,
        };
        guards_visited(
            &right_start,
            &right_end,
            &mut pts_visited,
            "right",
            &mut pts_loop,
            pts_v,
            max_len
        );
        let _bottom_end: &Points;
        let bottom_start = right_end;
        match bottom_start.find_bottom(&pts_v) {
            Some(value) => {
                _bottom_end = value;
            }
            None => {
                guards_visited(
                    &bottom_start,
                    &Points {
                        x: max_len as u8,
                        y: bottom_start.y,
                    },
                    &mut pts_visited,
                    "bottom",
                    &mut pts_loop,
                    pts_v,
                    max_len
                );
                break;
            }
        }
        let bottom_end = Points {
            x: _bottom_end.x - 1,
            y: _bottom_end.y,
        };
        guards_visited(
            &bottom_start,
            &bottom_end,
            &mut pts_visited,
            "bottom",
            &mut pts_loop,
            pts_v,
            max_len
        );
        let _left_end: &Points;
        let left_start = bottom_end;
        match left_start.find_left(&pts_v) {
            Some(value) => {
                _left_end = value;
            }
            None => {
                guards_visited(
                    &Points {
                        x: left_start.x,
                        y: 1,
                    },
                    &left_start,
                    &mut pts_visited,
                    "left",
                    &mut pts_loop,
                    pts_v,
                    max_len
                );
                break;
            }
        }
        let left_end = Points {
            x: _left_end.x,
            y: _left_end.y + 1,
        };
        guards_visited(
            &left_end,
            &left_start,
            &mut pts_visited,
            "left",
            &mut pts_loop,
            pts_v,
            max_len
        );
        top_start = left_end;
        // break;
    }
}

fn guards_visited(
    x_pt: &Points,
    y_pt: &Points,
    pts_visited: &mut HashSet<Points>,
    dir: &str,
    pts_loop: &mut HashSet<Points>,
    pts_v: &Vec<Points>,
    max_len: usize
) {
    for point in point_range(x_pt.clone(), y_pt.clone()) {
        pts_visited.insert(point.clone());
        insert_value(pts_loop, dir.to_string(), &point, pts_v, max_len);
    }
}

fn insert_value(map: &mut HashSet<Points>, key: String, value: &Points, pts_v: &Vec<Points>,  max_len: usize) {
    // If the key does not exist, insert a new HashSet
    let mut v = Points::default();
    if key == "left" {
        value.find_top(&pts_v).map(|_| v = Points { x: value.x, y: value.y - 1 }).unwrap_or_else(|| return)
    } else if key == "right" {
        value.find_bottom(&pts_v).map(|_| v = Points { x: value.x, y: value.y + 1 }).unwrap_or_else(|| return)
    } else if key == "top" {
        value.find_right(&pts_v).map(|_| v = Points { x: value.x - 1, y: value.y }).unwrap_or_else(|| return)
    } else if key == "bottom" {
        value.find_left(&pts_v).map(|_| v = Points { x: value.x + 1, y: value.y }).unwrap_or_else(|| return)
    }
    if v.x < 1 || v.x as usize > max_len || v.y < 1 || v.y as usize > max_len{
        // out of range skip
        return
    }
    map.insert(v);
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
