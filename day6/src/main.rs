mod helper;

use helper::{point_range, Points, read_points};

use std::collections::HashSet;
use std::time::Instant;
use std::io::{self};

fn main() -> io::Result<()> {
    let now = Instant::now();
    let (pts_v, mut curr_pt, max_len) = read_points("day6.txt").unwrap();

    let mut pts_visited = HashSet::new();
    loop {
        let _top_end: &Points;
        match curr_pt.find_top(&pts_v) {
            Some(value) => {
                _top_end = value;
            },
            None => {
                for point in point_range(Points{x:max_len as u8, y:curr_pt.y}, curr_pt) {
                    pts_visited.insert(point);
                };
                break;
            },
        }
        let top_end = Points {
            x: _top_end.x + 1,
            y: _top_end.y,
        };
        for point in point_range(top_end.clone(), curr_pt) {
            pts_visited.insert(point);
        };
        let _right_end: &Points;
        match top_end.find_right(&pts_v) {
            Some(value) => {
                _right_end = value;
            },
            None => {
                for point in point_range(top_end.clone(), Points{x:top_end.x, y:max_len as u8}) {
                    pts_visited.insert(point);
                };
                break;
            },
        }
        let right_end = Points {
            x: _right_end.x,
            y: _right_end.y - 1,
        };
        for point in point_range(top_end.clone(), right_end.clone()) {
            pts_visited.insert(point);
        }        
        let _bottom_end: &Points;
        match right_end.find_bottom(&pts_v) {
            Some(value) => {
                _bottom_end = value;
            },
            None => {
                for point in point_range(right_end.clone(), Points{x:max_len as u8, y:right_end.y}) {
                    pts_visited.insert(point);
                };
                break;
            },
        }
        let bottom_end = Points {
            x: _bottom_end.x - 1,
            y: _bottom_end.y,
        };
        for point in point_range(right_end.clone(), bottom_end.clone()) {
            pts_visited.insert(point);
        }
        let _left_end: &Points;
        match bottom_end.find_left(&pts_v) {
            Some(value) => {
                _left_end = value;
            },
            None => {
                for point in point_range(Points{x:bottom_end.x, y:max_len as u8}, bottom_end.clone()) {
                    pts_visited.insert(point);
                };
                break;
            },
        }
        let left_end = Points {
            x: _left_end.x,
            y: _left_end.y + 1,
        };
        for point in point_range(left_end.clone(), bottom_end.clone()) {
            pts_visited.insert(point);
        }
        curr_pt = left_end;
    }

    println!("Day6 Part 1: {}",pts_visited.len());
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    Ok(())
}
