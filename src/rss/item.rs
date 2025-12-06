use derive_builder::Builder;
use serde::Serialize;
use serde_with::skip_serializing_none;
use crate::rss::category::Category;
use crate::rss::enclosure::Enclosure;
use crate::rss::item_guid::ItemGuid;
use crate::rss::item_source::ItemSource;

#[skip_serializing_none]
#[derive(Serialize, Builder, Clone, Default)]
#[builder(build_fn(private, name = "fallible_build"), setter(into, strip_option), default)]
#[serde(rename = "item", rename_all = "camelCase")]
pub struct Item {
    title: Option<String>,
    link: Option<String>,
    description: Option<String>,
    author: Option<String>,
    category: Option<Category>,
    comments: Option<String>,
    enclosure: Option<Enclosure>,
    guid: Option<ItemGuid>,
    pub_date: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::{Item,ItemBuilder};
    use quick_xml::se::to_string;
    use crate::rss::category::CategoryBuilder;
    use crate::rss::enclosure::Enclosure;
    use crate::rss::item_guid::ItemGuidBuilder;
    use crate::rss::item_source::ItemSourceBuilder;

    fn mk_item_with_title() -> Item {
        ItemBuilder::with_title("title").build()
    }

    fn mk_item_with_description() -> Item {
        ItemBuilder::with_description("description").build()
    }

    #[test]
    fn no_required_fields_added_to_item() {
        mk_item_with_title();
        mk_item_with_description();
    }

    #[test]
    fn test_serialize_with_title() {
        let item = mk_item_with_title();
        assert_eq!(
            to_string(&item).unwrap(),
            "<item>\
                <title>title</title>\
            </item>"
        );
    }

    #[test]
    fn test_serialize_with_description() {
        let item = mk_item_with_description();
        assert_eq!(
            to_string(&item).unwrap(),
            "<item>\
                <description>description</description>\
            </item>"
        );
    }

    fn mk_item_with_all_fields() -> Item {
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
            .build()
    }

    #[test]
    fn test_serialize_with_all_fields() {
        let item = mk_item_with_all_fields();
        assert_eq!(
            to_string(&item).unwrap(),
            "<item>\
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
            </item>"
        );
    }
}