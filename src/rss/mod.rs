use derive_builder::Builder;
use serde::{Serialize, Serializer};

mod item_guid;
mod category;
mod enclosure;
mod item_source;
mod item;
mod image;
mod channel;

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

#[derive(Serialize, Builder, Clone)]
#[builder(build_fn(private, name = "fallible_build"))]
pub struct Channel {
    #[builder(setter(into))]
    title: String,

    #[builder(setter(into))]
    link: String,

    #[builder(setter(into))]
    description: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    copyright: Option<String>,

    #[serde(rename = "managingEditor", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    managing_editor: Option<String>,

    #[serde(rename = "webMaster", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    web_master: Option<String>,

    #[serde(rename = "pubDate", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pub_date: Option<String>,

    #[serde(rename = "lastBuildDate", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    last_build_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    category: Option<Category>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    generator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    docs: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // cloud: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    ttl: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    image: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    rating: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // textInput: Option<String>,
    #[serde(rename = "skipHours", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    skip_hours: Option<u8>,

    #[serde(rename = "skipDays", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    skip_days: Option<String>,

    #[builder(default)]
    item: Vec<Item>,
}

impl ChannelBuilder {
    pub fn new<T: Into<String>, L: Into<String>, D: Into<String>>(
        title: T,
        link: L,
        description: D,
    ) -> Self {
        ChannelBuilder::default()
            .title(title)
            .link(link)
            .description(description)
            .to_owned()
    }

    pub fn build(&mut self) -> Channel {
        self.fallible_build().expect("All required fields set.")
    }
}

#[derive(Serialize, Builder, Clone)]
#[builder(build_fn(private, name = "fallible_build"))]
pub struct Image {
    #[builder(setter(into))]
    url: String,

    #[builder(setter(into))]
    title: String,

    #[builder(setter(into))]
    link: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    description: Option<String>,
}

impl ImageBuilder {
    pub fn new<U: Into<String>, T: Into<String>, L: Into<String>>(
        url: U,
        title: T,
        link: L,
    ) -> Self {
        ImageBuilder::default()
            .url(url)
            .title(title)
            .link(link)
            .to_owned()
    }

    pub fn build(&mut self) -> Image {
        self.fallible_build().expect("All required fields set.")
    }
}

#[derive(Serialize, Builder, Clone)]
#[builder(build_fn(private, name = "fallible_build"))]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    author: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    category: Option<Category>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    comments: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    enclosure: Option<Enclosure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    guid: Option<ItemGuid>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "pubDate")]
    #[builder(setter(into, strip_option), default)]
    pub_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    source: Option<ItemSource>,
}

impl ItemBuilder {
    pub fn with_title<T: Into<String>>(title: T) -> Self {
        ItemBuilder::default().title(title).to_owned()
    }

    pub fn with_description<T: Into<String>>(description: T) -> Self {
        ItemBuilder::default().description(description).to_owned()
    }

    pub fn build(&mut self) -> Item {
        self.fallible_build().expect("All required fields set.")
    }
}

#[derive(Serialize, Builder, Clone)]
#[builder(build_fn(private, name = "fallible_build"))]
pub struct ItemSource {
    #[serde(rename = "@url")]
    #[builder(setter(into))]
    pub url: String,

    #[serde(rename = "$text")]
    #[builder(setter(into, strip_option), default)]
    pub text: Option<String>,
}

impl ItemSourceBuilder {
    pub fn new<T: Into<String>>(url: T) -> ItemSourceBuilder {
        ItemSourceBuilder::default().url(url).to_owned()
    }

    pub fn build(&self) -> ItemSource {
        self.fallible_build().expect("All required fields set.")
    }
}

#[derive(Serialize, Clone)]
pub struct Enclosure {
    #[serde(rename = "@url")]
    url: String,
    #[serde(rename = "@length")]
    length: u64,
    #[serde(rename = "@type")]
    media_type: String,
}

impl Enclosure {
    pub fn new<U: Into<String>, M: Into<String>>(url: U, length: u64, media_type: M) -> Self {
        Enclosure{
            url: url.into(),
            length,
            media_type: media_type.into()
        }
    }
}

#[derive(Serialize, Builder, Clone)]
#[builder(build_fn(private, name = "fallible_build"))]
pub struct Category {
    #[serde(rename = "@domain", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    domain: Option<String>,

    #[serde(rename = "$text")]
    #[builder(setter(into))]
    text: String,
}

impl CategoryBuilder {
    pub fn new<T: Into<String>>(text: T) -> Self {
        CategoryBuilder::default().text(text).to_owned()
    }

    pub fn build(&mut self) -> Category {
        self.fallible_build().expect("All required fields set.")
    }
}

#[derive(Serialize, Builder, Clone)]
#[builder(build_fn(private, name = "fallible_build"))]
pub struct ItemGuid {
    #[serde(rename = "@isPermalink", skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    is_permalink: Option<bool>,

    #[serde(rename = "$text")]
    #[builder(setter(into))]
    guid: String,
}

impl ItemGuidBuilder {
    pub fn new<T: Into<String>>(guid: T) -> Self {
        ItemGuidBuilder::default().guid(guid).to_owned()
    }

    pub fn build(&mut self) -> ItemGuid {
        self.fallible_build().expect("All required fields set")
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
                .language("en")
                // copyright: None,
                // managing_editor: None,
                // web_master: None,
                // pub_date: None,
                // last_build_date: None,
                // category: None,
                // generator: None,
                // docs: None,
                // ttl: None,
                .image(
                    ImageBuilder::new(
                        "https://some/channel/image.png",
                        "The Channel Image",
                        "https://link/to/channe/from/image",
                    )
                    .build(),
                )
                // .rating(: None,
                // .skip_hours(: None,
                // .skip_days(: None,
                .item( vec![
                    ItemBuilder::with_title("Item")
                        .link("https://www.google.com")
                        .description("An item description")
                        .author("The Author".to_string())
                        .category(
                            CategoryBuilder::new("category")
                                .domain("https://category.domain")
                                .build(),
                        )
                        .comments("https://some.com/link-to-comments")
                        .enclosure(Enclosure::new(
                            "https://enclosure/url.mp3",
                            1234,
                            "audio/mpeg",
                        ))
                        .guid(
                            ItemGuidBuilder::new("https://inessential.com/123")
                                .is_permalink(true)
                                .build(),
                        )
                        .pub_date("2015-01-01T00:00:00Z")
                        .source(
                            ItemSourceBuilder::new("https://inessential.com/123")
                                .text("The Source")
                                .build(),
                        )
                        .build(),
                ])
                .build()
        };
        assert_eq!(
            to_string(&data).unwrap(),
            "<rss version=\"2.0\">\
                <channel>\
                    <title>some-title</title>\
                    <link>https://www.google.com</link>\
                    <description>A description: such &amp; such.</description>\
                    <language>en</language>\
                    <image>\
                        <url>https://some/channel/image.png</url>\
                        <title>The Channel Image</title>\
                        <link>https://link/to/channe/from/image</link>\
                    </image>\
                    <item>\
                        <title>Item</title>\
                        <link>https://www.google.com</link>\
                        <description>An item description</description>\
                        <author>The Author</author>\
                        <category domain=\"https://category.domain\">category</category>\
                        <comments>https://some.com/link-to-comments</comments>\
                        <enclosure url=\"https://enclosure/url.mp3\" length=\"1234\" type=\"audio/mpeg\"/>\
                        <guid isPermalink=\"true\">https://inessential.com/123</guid>\
                        <pubDate>2015-01-01T00:00:00Z</pubDate>\
                        <source url=\"https://inessential.com/123\">The Source</source>\
                    </item>\
                </channel>\
            </rss>"
        );
    }

    #[test]
    fn test_skips_serialization_of_option() {
        let data = Rss {
            version: RssVersion::RSS2_0,
            channel: Channel {
                title: "some-title".to_string(),
                link: "https://www.google.com".to_string(),
                description: "A description: such & such.".to_string(),
                language: None,
                copyright: None,
                managing_editor: None,
                web_master: None,
                pub_date: None,
                last_build_date: None,
                category: None,
                generator: None,
                docs: None,
                ttl: None,
                image: None,
                rating: None,
                skip_hours: None,
                skip_days: None,
                item: vec![],
            },
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
