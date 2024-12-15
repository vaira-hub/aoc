mod day6;

use day6::day6;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    day6();
    println!("Elapsed: {:.2?}", now.elapsed());
}
