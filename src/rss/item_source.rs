use derive_builder::Builder;
use serde::Serialize;

#[serde_with::apply(
    Option => #[builder(default)] #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Builder, Clone, PartialEq, Debug)]
#[builder(build_fn(private, name = "fallible_build"), setter(into))]
#[serde(rename = "source")]
pub struct ItemSource {
    #[serde(rename = "@url")]
    pub url: String,

    #[serde(rename = "$text")]
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

#[cfg(test)]
mod tests {
    use super::ItemSourceBuilder;
    use quick_xml::se::to_string;

    #[test]
    fn no_required_fields_added_to_constructor() {
        ItemSourceBuilder::new("url").build();
    }

    #[test]
    fn test_serialize_without_text() {
        let source = ItemSourceBuilder::new("url").build();
        assert_eq!(
            to_string(&source).unwrap(),
            "<source url=\"url\"/>"
        );
    }

    #[test]
    fn test_serialize_with_text() {
        let source = ItemSourceBuilder::new("url").text("some text".to_owned()).build();
        assert_eq!(
            to_string(&source).unwrap(),
            "<source url=\"url\">some text</source>"
        );
    }
}