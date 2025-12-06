use serde::{Serialize, Serializer};

mod item_guid;
mod category;
mod enclosure;
mod item_source;
mod item;
mod image;
mod channel;

pub use item_guid::*;
pub use category::*;
pub use enclosure::*;
pub use item_source::*;
pub use item::*;
pub use image::*;
pub use channel::*;

#[derive(Serialize)]
#[serde(rename = "rss")]
pub struct Rss {
    #[serde(rename = "@version")]
    version: RssVersion,
    channel: Channel,
}

impl Rss {
    pub fn new(channel: Channel) -> Self {
        Rss {
            version: RssVersion::RSS2_0,
            channel
        }
    }
}

pub enum RssVersion {
    RSS2_0,
}
impl RssVersion {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::RSS2_0 => "2.0",
        }
    }
}

impl Serialize for RssVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::se::to_string;

    #[test]
    fn test_serialize() {
        let data = Rss {
            version: RssVersion::RSS2_0,
            channel: ChannelBuilder::new(
                    "some-title",
                    "https://www.google.com",
                    "A description: such & such."
                )
                .build()
        };
        assert_eq!(
            to_string(&data).unwrap(),
            "<rss version=\"2.0\">\
                <channel>\
                    <title>some-title</title>\
                    <link>https://www.google.com</link>\
                    <description>A description: such &amp; such.</description>\
                </channel>\
            </rss>"
        );
    }
}
