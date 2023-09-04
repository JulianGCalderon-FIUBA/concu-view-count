use csv::StringRecord;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Video {
    pub channel: String,
    pub views: usize,
}

impl TryFrom<StringRecord> for Video {
    type Error = Box<dyn std::error::Error>;

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        let channel = value.get(3).ok_or("No channel title")?.to_string();
        let views = value.get(7).ok_or("No views")?.parse()?;

        Ok(Self { channel, views })
    }
}
