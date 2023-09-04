use csv::Reader;
use glob::glob;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use view_count::utils::{add_key, merge_maps, time};
use view_count::video::Video;

fn main() {
    let (time, ret) = time(run);
    println!("Parallel: {:?}", time);
    println!("Channels: {:?}", ret.keys().len());
    println!("MasterChef 2017: {:?}", ret.get("MasterChef 2017"));
}

fn run() -> HashMap<String, usize> {
    glob("data/*")
        .unwrap()
        .par_bridge()
        .flatten()
        .flat_map(Reader::from_path)
        .map(|reader| parse_file(reader))
        .reduce(|| HashMap::new(), merge_maps)
}

fn parse_file(reader: Reader<File>) -> HashMap<String, usize> {
    reader
        .into_records()
        .flatten()
        .flat_map(Video::try_from)
        .fold(HashMap::new(), add_key)
}
