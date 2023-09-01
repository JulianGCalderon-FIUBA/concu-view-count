use csv::Reader;
use glob::glob;
use std::collections::HashMap;

use crate::util::{Video, HEADER_RECORD};

pub fn sequential() -> HashMap<String, usize> {
    glob("data/*")
        .unwrap()
        .flatten()
        .flat_map(|path| Reader::from_path(path))
        .map(|reader| reader.into_records())
        .flatten()
        .flatten()
        .flat_map(|row| row.deserialize::<Video>(Some(&HEADER_RECORD)))
        .fold(HashMap::new(), |mut acc, video: Video| {
            let entry = acc.entry(video.channel_title).or_insert(0);
            *entry += video.views;
            acc
        })
}
