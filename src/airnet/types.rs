use serde::{de, Deserialize, Deserializer};
use chrono::{NaiveDateTime};

#[derive(Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Deserialize, Debug, PartialEq, Clone)]
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

fn naive_date_time_from_str<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").map_err(de::Error::custom)
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Episode {
    pub url: Option<String>,
    #[serde(deserialize_with = "naive_date_time_from_str")]
    pub start: NaiveDateTime,
    #[serde(deserialize_with = "naive_date_time_from_str")]
    pub end: NaiveDateTime,
    pub duration: u32,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename="imageUrl")]
    pub image_url: Option<String>,
    #[serde(rename="episodeRestUrl")]
    pub episode_rest_url: String,
}
