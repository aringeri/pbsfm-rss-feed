use derive_builder::Builder;
use serde::Serialize;

#[serde_with::apply(
    Option => #[builder(default)] #[serde(skip_serializing_if = "Option::is_none")],
)]
#[derive(Serialize, Builder, Clone, PartialEq, Debug)]
#[serde(rename = "category")]
#[builder(build_fn(private, name = "fallible_build"), setter(into))]
pub struct Category {
    #[serde(rename = "@domain")]
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
    fn no_required_fields_added_to_constructor() {
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
        let category = CategoryBuilder::new("my category").domain("some domain".to_owned()).build();
        assert_eq!(
            to_string(&category).unwrap(),
            "<category domain=\"some domain\">my category</category>"
        );
    }
}