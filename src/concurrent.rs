use csv::Reader;
use glob::glob;
use rayon::prelude::*;
use std::collections::HashMap;

use view_count::utils::{add_key, merge_maps, time};
use view_count::video::Video;

fn run() -> HashMap<String, usize> {
    glob("data/*")
        .unwrap()
        .par_bridge()
        .flatten()
        .flat_map(|path| Reader::from_path(path))
        .flat_map(|reader| reader.into_records().par_bridge().flatten())
        .map(Video::from)
        .fold(HashMap::new, add_key)
        .reduce(HashMap::new, merge_maps)
}

pub fn main() {
    let (time, ret) = time(run);
    println!("Parallel: {:?}", time);
    println!("Channels: {:?}", ret.keys().len());
    println!("MasterChef 2017: {:?}", ret.get("MasterChef 2017"));
}
