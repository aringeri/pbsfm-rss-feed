use crate::rss::category::Category;
use crate::rss::image::Image;
use crate::rss::item::Item;
use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;

#[serde_with::apply(
    Option => #[builder(default)],
    Vec => #[builder(default)],
)]
#[skip_serializing_none]
#[derive(Serialize, Builder, Clone, PartialEq, Debug)]
#[builder(build_fn(private, name = "fallible_build"), setter(into, strip_option))]
#[serde(rename = "channel", rename_all = "camelCase")]
pub struct Channel {
    title: String,
    link: String,
    description: String,
    language: Option<String>,
    copyright: Option<String>,
    managing_editor: Option<String>,
    web_master: Option<String>,
    pub_date: Option<String>,
    last_build_date: Option<String>,
    category: Option<Category>,
    generator: Option<String>,
    docs: Option<String>,
    // cloud: Option<String>,
    ttl: Option<u64>,
    image: Option<Image>,
    rating: Option<String>,
    // textInput: Option<String>,
    skip_hours: Option<u8>,
    skip_days: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::ChannelBuilder;
    use crate::rss::category::CategoryBuilder;
    use crate::rss::enclosure::Enclosure;
    use crate::rss::image::ImageBuilder;
    use crate::rss::item::ItemBuilder;
    use crate::rss::item_guid::ItemGuidBuilder;
    use crate::rss::item_source::ItemSourceBuilder;
    use quick_xml::se::to_string;

    #[test]
    fn no_required_fields_added_to_constructor() {
        ChannelBuilder::new("title", "link", "description").build();
    }

    #[test]
    fn test_serialize_with_minimum_params() {
        let channel = ChannelBuilder::new("title", "link", "description").build();
        assert_eq!(
            to_string(&channel).unwrap(),
            "<channel>\
                <title>title</title>\
                <link>link</link>\
                <description>description</description>\
            </channel>"
        );
    }

    #[test]
    fn test_serialize_with_all_params() {
        let channel = ChannelBuilder::new(
            "some-title",
            "https://www.google.com",
            "A description: such & such.",
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
        .item(vec![
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
        .build();
        assert_eq!(
            to_string(&channel).unwrap(),
            "<channel>\
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
            </channel>"
        );
    }
}
