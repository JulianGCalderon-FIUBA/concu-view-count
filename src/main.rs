mod parallel;
mod sequential;
mod util;

fn main() {
    let (seq_time, seq_ret) = util::time(sequential::sequential);
    println!("Sequential: {:?}", seq_time);
    println!("MasterChef 2017: {:?}", seq_ret.get("MasterChef 2017"));

    let (par_time, par_ret) = util::time(parallel::parallel);
    println!("Parallel: {:?}", par_time);
    println!("MasterChef 2017: {:?}", par_ret.get("MasterChef 2017"));
}
