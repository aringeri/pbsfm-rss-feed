use derive_builder::Builder;
use serde::{Serialize};

#[serde_with::apply(
    Option => #[builder(default)] #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Builder, Clone, PartialEq, Debug)]
#[builder(build_fn(private, name = "fallible_build"), setter(into))]
#[serde(rename = "guid")]
pub struct ItemGuid {
    #[serde(rename = "@isPermalink")]
    is_permalink: Option<bool>,

    #[serde(rename = "$text")]
    guid: String,
}

impl ItemGuidBuilder {
    pub fn new<S: Into<String>>(guid: S) -> Self {
        ItemGuidBuilder::default().guid(guid).to_owned()
    }

    pub fn build(&mut self) -> ItemGuid {
        self.fallible_build().expect("All required fields set")
    }
}

#[cfg(test)]
mod tests {
    use super::ItemGuidBuilder;
    use quick_xml::se::to_string;

    #[test]
    fn no_required_fields_added_to_constructor() {
        ItemGuidBuilder::new("guid").build();
    }

    #[test]
    fn test_serialize_without_permalink() {
        let guid = ItemGuidBuilder::new("https://guid").build();
        assert_eq!(
            to_string(&guid).unwrap(),
            "<guid>https://guid</guid>"
        );
    }

    #[test]
    fn test_serialize_with_permalink() {
        let guid = ItemGuidBuilder::new("https://guid").is_permalink(true).build();
        assert_eq!(
            to_string(&guid).unwrap(),
            "<guid isPermalink=\"true\">https://guid</guid>"
        );
    }

}