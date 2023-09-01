use std::time::{Duration, Instant};

use csv::StringRecord;
use lazy_static::lazy_static;
use serde::Deserialize;

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
    pub static ref HEADER_RECORD: StringRecord = StringRecord::from(HEADERS);
}

#[derive(Debug, Deserialize)]
pub struct Video {
    pub channel_title: String,
    pub views: usize,
}

pub fn time<R, F: Fn() -> R>(f: F) -> (Duration, R) {
    let start = Instant::now();
    let ret = f();
    (start.elapsed(), ret)
}
