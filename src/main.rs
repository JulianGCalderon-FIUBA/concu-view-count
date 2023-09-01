use glob::glob;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use csv::{Reader, StringRecord};

const HEADERS_SLICE: &[&str] = &[
    "video_id",
    "trending_date",
    "title",
    "channel_title",
    "category_id",
    "publish_time",
    "tags",
    "views",
    "likes",
    "dislikes",
    "comment_count",
    "thumbnail_link",
    "comments_disabled",
    "ratings_disabled",
    "video_error_or_removed",
    "description",
];

lazy_static! {
    static ref HEADERS: StringRecord = StringRecord::from(HEADERS_SLICE);
}

#[derive(Debug, Deserialize)]
struct Video {
    channel_title: String,
    views: usize,
}

fn sequential() -> HashMap<String, usize> {
    glob("data/*")
        .unwrap()
        .flatten()
        .flat_map(|path| Reader::from_path(path))
        .map(|reader| reader.into_records())
        .flatten()
        .flatten()
        .flat_map(|row| row.deserialize::<Video>(Some(&HEADERS)))
        .fold(HashMap::new(), |mut acc, video: Video| {
            let entry = acc.entry(video.channel_title).or_insert(0);
            *entry += video.views;
            acc
        })
}

fn parallel() -> HashMap<String, usize> {
    glob("data/*")
        .unwrap()
        .par_bridge()
        .flatten()
        .flat_map(|path| Reader::from_path(path))
        .map(|reader| reader.into_records().par_bridge())
        .flatten()
        .flatten()
        .flat_map(|row| row.deserialize::<Video>(Some(&HEADERS)))
        .fold(
            || HashMap::new(),
            |mut acc, video: Video| {
                let entry = acc.entry(video.channel_title).or_insert(0);
                *entry += video.views;
                acc
            },
        )
        .reduce(
            || HashMap::new(),
            |mut acc, map| {
                for (key, value) in map {
                    let entry = acc.entry(key).or_insert(0);
                    *entry += value;
                }
                acc
            },
        )
}

fn time<R, F: Fn() -> R>(f: F) -> (Duration, R) {
    let start = Instant::now();
    let ret = f();

    (start.elapsed(), ret)
}

fn main() {
    let (seq_time, seq_ret) = time(sequential);
    println!("Sequential: {:?}", seq_time);
    println!("MasterChef 2017: {:?}", seq_ret.get("MasterChef 2017"));

    let (par_time, par_ret) = time(parallel);
    println!("Parallel: {:?}", par_time);
    println!("MasterChef 2017: {:?}", par_ret.get("MasterChef 2017"));
}
