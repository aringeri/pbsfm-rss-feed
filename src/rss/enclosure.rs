use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Debug)]
#[serde(rename = "enclosure")]
pub struct Enclosure {
    #[serde(rename = "@url")]
    url: String,
    #[serde(rename = "@length", skip_serializing_if = "Option::is_none")]
    length: Option<u64>,
    #[serde(rename = "@type")]
    media_type: String,
}

impl Enclosure {
    pub fn new<U: Into<String>, M: Into<String>>(url: U, length: Option<u64>, media_type: M) -> Self {
        Enclosure {
            url: url.into(),
            length,
            media_type: media_type.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Enclosure;
    use quick_xml::se::to_string;

    #[test]
    fn test_serialize() {
        let enclosure = Enclosure::new(
            "url",
            Some(182),
            "media-type"
        );
        assert_eq!(
            to_string(&enclosure).unwrap(),
            "<enclosure url=\"url\" length=\"182\" type=\"media-type\"/>"
        );
    }

    #[test]
    fn test_serialize_without_length() {
        let enclosure = Enclosure::new(
            "url",
            None,
            "media-type"
        );
        assert_eq!(
            to_string(&enclosure).unwrap(),
            "<enclosure url=\"url\" type=\"media-type\"/>"
        );
    }
}