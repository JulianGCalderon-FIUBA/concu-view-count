use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Video {
    pub channel_title: String,
    pub views: usize,
}

impl From<StringRecord> for Video {
    fn from(value: StringRecord) -> Self {
        Self {
            channel_title: value.get(3).unwrap().to_string(),
            views: value.get(7).unwrap().parse().unwrap(),
        }
    }
}
