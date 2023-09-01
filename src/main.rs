mod parallel;
mod sequential;

use glob::glob;
use lazy_static::lazy_static;
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use csv::{Reader, StringRecord};

const HEADERS: &[&str] = &[
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
    static ref HEADER_RECORD: StringRecord = StringRecord::from(HEADERS);
}

#[derive(Debug, Deserialize)]
struct Video {
    channel_title: String,
    views: usize,
}

fn time<R, F: Fn() -> R>(f: F) -> (Duration, R) {
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
