use derive_builder::Builder;
use serde::Serialize;

#[derive(Serialize, Builder, Clone)]
#[serde(rename = "category")]
#[builder(build_fn(private, name = "fallible_build"), setter(into, strip_option))]
pub struct Category {
    #[serde(rename = "@domain", skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    domain: Option<String>,

    #[serde(rename = "$text")]
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

#[cfg(test)]
mod tests {
    use super::CategoryBuilder;
    use quick_xml::se::to_string;

    #[test]
    fn no_required_fields_added_to_category() {
        CategoryBuilder::new("category").build();
    }

    #[test]
    fn test_serialize_without_domain() {
        let category = CategoryBuilder::new("my category").build();
        assert_eq!(
            to_string(&category).unwrap(),
            "<category>my category</category>"
        );
    }

    #[test]
    fn test_serialize_with_domain() {
        let category = CategoryBuilder::new("my category").domain("some domain").build();
        assert_eq!(
            to_string(&category).unwrap(),
            "<category domain=\"some domain\">my category</category>"
        );
    }
}