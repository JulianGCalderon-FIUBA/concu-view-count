use std::collections::HashMap;
use std::time::{Duration, Instant};

use crate::video::Video;

pub fn add_key(mut acc: HashMap<String, usize>, video: Video) -> HashMap<String, usize> {
    let entry = acc.entry(video.channel).or_insert(0);
    *entry += video.views;
    acc
}

pub fn merge_maps(
    mut map1: HashMap<String, usize>,
    map2: HashMap<String, usize>,
) -> HashMap<String, usize> {
    for (key, value) in map2 {
        let entry = map1.entry(key).or_insert(0);
        *entry += value;
    }
    map1
}

pub fn time<R, F: Fn() -> R>(f: F) -> (Duration, R) {
    let start = Instant::now();
    let ret = f();
    (start.elapsed(), ret)
}
