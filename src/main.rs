use std::time::{Duration, Instant};

mod parallel;
mod sequential;
mod util;

pub fn time<R, F: Fn() -> R>(f: F) -> (Duration, R) {
    let start = Instant::now();
    let ret = f();
    (start.elapsed(), ret)
}

fn main() {
    let (seq_time, seq_ret) = time(sequential::sequential);
    println!("Sequential: {:?}", seq_time);
    println!("MasterChef 2017: {:?}", seq_ret.get("MasterChef 2017"));

    let (par_time, par_ret) = time(parallel::parallel);
    println!("Parallel: {:?}", par_time);
    println!("MasterChef 2017: {:?}", par_ret.get("MasterChef 2017"));
}
