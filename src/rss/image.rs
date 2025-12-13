use derive_builder::Builder;
use serde::Serialize;

#[serde_with::apply(
    Option => #[builder(default)] #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Builder, Clone, PartialEq, Debug)]
#[builder(build_fn(private, name = "fallible_build"), setter(into))]
#[serde(rename = "image")]
pub struct Image {
    url: String,
    title: String,
    link: String,
    width: Option<u32>,
    height: Option<u32>,
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

#[cfg(test)]
mod tests {
    use super::ImageBuilder;
    use quick_xml::se::to_string;

    #[test]
    fn no_required_fields_added_to_constructor() {
        ImageBuilder::new("url", "title", "link").build();
    }

    #[test]
    fn test_serialize_with_minimum_params() {
        let image = ImageBuilder::new("url", "title", "link").build();
        assert_eq!(
            to_string(&image).unwrap(),
            "<image>\
                <url>url</url>\
                <title>title</title>\
                <link>link</link>\
            </image>"
        );
    }

    #[test]
    fn test_serialize_with_all_params() {
        let image = ImageBuilder::new("url", "title", "link")
            .width(1u32)
            .height(2u32)
            .description("description".to_owned())
            .build();
        assert_eq!(
            to_string(&image).unwrap(),
            "<image>\
                <url>url</url>\
                <title>title</title>\
                <link>link</link>\
                <width>1</width>\
                <height>2</height>\
                <description>description</description>\
            </image>"
        );
    }
}