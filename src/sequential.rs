use csv::Reader;
use glob::glob;
use std::collections::HashMap;

use view_count::utils::{add_key, time};
use view_count::video::Video;

fn run() -> HashMap<String, usize> {
    glob("data/*")
        .unwrap()
        .flatten()
        .flat_map(|path| Reader::from_path(path))
        .map(|reader| reader.into_records().flatten())
        .flatten()
        .flat_map(Video::try_from)
        .fold(HashMap::new(), add_key)
}

fn main() {
    let (time, ret) = time(run);
    println!("Parallel: {:?}", time);
    println!("Channels: {:?}", ret.keys().len());
    println!("MasterChef 2017: {:?}", ret.get("MasterChef 2017"));
}
