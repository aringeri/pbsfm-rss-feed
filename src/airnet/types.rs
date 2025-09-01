use serde::Deserialize;
use time::PrimitiveDateTime;

#[derive(Deserialize, Debug, PartialEq)]
pub struct ProgramDescription {
    pub slug: Option<String>,
    pub name: String,
    pub broadcasters: String,
    #[serde(rename="gridDescription")]
    pub grid_description: Option<String>,
    pub archived: bool,
    #[serde(rename="programRestUrl")]
    pub program_rest_url: String
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ProgramDetails {
    pub url: Option<String>,
    pub name: String,
    pub broadcasters: String,
    pub description: String,
    #[serde(rename="gridDescription")]
    pub grid_description: Option<String>,
    pub slug: String,
    #[serde(rename="bannerImageUrl")]
    pub banner_image_url: String,
    #[serde(rename="bannerImageSmall")]
    pub banner_image_small: String,
    #[serde(rename="profileImageUrl")]
    pub profile_image_url: String,
    #[serde(rename="profileImageSmall")]
    pub profile_image_small: String,
    #[serde(rename="episodesRestUrl")]
    pub episodes_rest_url: String,
}

pub struct Episode {
    pub url: Option<String>,
    pub start: PrimitiveDateTime,
    pub end: PrimitiveDateTime,
    pub duration: u32,
    pub title: String,
    pub description: Option<String>,
    pub imageUrl: Option<String>,
    pub episodeRestUrl: Option<String>,
}
