use csv::Reader;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use view_count::utils::time;

fn main() {
    let (time, ret) = time(run);
    println!("Parallel: {:?}", time);
    println!("Channels: {:?}", ret.keys().len());
    println!("MasterChef 2017: {:?}", ret.get("MasterChef 2017"));
}

fn run() -> HashMap<String, i64> {
    process([
        "CAvideos", "DEvideos", "FRvideos", "GBvideos", "INvideos", "JPvideos", "KRvideos",
        "MXvideos", "RUvideos", "USvideos",
    ])
}

pub fn process(paths: [&str; 10]) -> HashMap<String, i64> {
    paths
        .par_iter()
        .map(|p| process_file(format!("./data/{}.csv", p)))
        .reduce(
            || HashMap::new(),
            |h1: HashMap<String, i64>, h2: HashMap<String, i64>| hash_merging(&h1, &h2),
        )
}

fn hash_merging(h1: &HashMap<String, i64>, h2: &HashMap<String, i64>) -> HashMap<String, i64> {
    if h1.is_empty() {
        return h2.to_owned();
    }

    if h2.is_empty() {
        return h2.to_owned();
    }

    let mut h = h1.to_owned();

    for (key, val) in h2.iter() {
        add_hashmap_key(&mut h, key.to_string(), val.to_owned());
    }

    h
}

fn add_hashmap_key(h: &mut HashMap<String, i64>, k: String, v: i64) {
    if h.contains_key(&k) {
        let v_old = h.get(&k).unwrap();
        h.insert(k, v_old + v);
    } else {
        h.insert(k, v);
    }
}

fn process_file(filename: String) -> HashMap<String, i64> {
    let mut rdr = Reader::from_path(&filename).unwrap();
    let mut data: HashMap<String, i64> = HashMap::new();
    for result in rdr.records() {
        if result.is_err() {
            // ignore
            continue;
        }

        let record = result.unwrap();
        let channel_title = record.get(3).expect("error").to_string();
        let channel_view = record.get(7).expect("error").parse::<i64>().expect("err");

        add_hashmap_key(&mut data, channel_title, channel_view);
    }

    data
}
