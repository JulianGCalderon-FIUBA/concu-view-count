pub fn parallel() -> HashMap<String, usize> {
    glob("data/*")
        .unwrap()
        .par_bridge()
        .flatten()
        .flat_map(|path| Reader::from_path(path))
        .map(|reader| reader.into_records().par_bridge())
        .flatten()
        .flatten()
        .flat_map(|row| row.deserialize::<Video>(Some(&HEADER_RECORD)))
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
