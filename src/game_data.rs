use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize, de};
use std::collections::HashMap;

pub type GameMap = HashMap<u64, Game>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    #[serde(alias = "_firestore_id")]
    id: Option<String>,
    #[serde(rename = "steam_app_id")]
    pub steam_app_id: Option<u64>,
    pub name: String,
    #[serde(rename = "release_date")]
    pub release_date: String,
    #[serde(rename = "required_age")]
    pub required_age: i64,
    pub price: f64,
    #[serde(rename = "dlc_count")]
    pub dlc_count: i64,
    #[serde(rename = "detailed_description")]
    pub detailed_description: String,
    #[serde(rename = "about_the_game")]
    pub about_the_game: String,
    #[serde(rename = "short_description")]
    pub short_description: String,
    pub reviews: String,
    #[serde(rename = "header_image")]
    pub header_image: String,
    pub website: String,
    #[serde(rename = "support_url")]
    pub support_url: String,
    #[serde(rename = "support_email")]
    pub support_email: String,
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
    #[serde(rename = "metacritic_score")]
    pub metacritic_score: i64,
    #[serde(rename = "metacritic_url")]
    pub metacritic_url: String,
    pub achievements: i64,
    pub recommendations: i64,
    pub notes: String,
    #[serde(rename = "supported_languages")]
    pub supported_languages: Vec<String>,
    #[serde(rename = "full_audio_languages")]
    pub full_audio_languages: Vec<String>,
    pub packages: Vec<Package>,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub categories: Vec<String>,
    pub genres: Vec<String>,
    pub screenshots: Vec<String>,
    pub movies: Vec<String>,
    #[serde(rename = "user_score")]
    pub user_score: i64,
    #[serde(rename = "score_rank")]
    pub score_rank: StringOrU64,
    pub positive: i64,
    pub negative: i64,
    #[serde(rename = "estimated_owners")]
    pub estimated_owners: String,
    #[serde(rename = "average_playtime_forever")]
    pub average_playtime_forever: i64,
    #[serde(rename = "average_playtime_2weeks")]
    pub average_playtime_2weeks: i64,
    #[serde(rename = "median_playtime_forever")]
    pub median_playtime_forever: i64,
    #[serde(rename = "median_playtime_2weeks")]
    pub median_playtime_2weeks: i64,
    #[serde(rename = "peak_ccu")]
    pub peak_ccu: i64,
    #[serde(deserialize_with = "deserialize_tags")]
    pub tags: HashMap<String, u64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub title: String,
    pub description: String,
    pub subs: Vec<Sub>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct Sub {
    pub text: String,
    pub description: String,
    pub price: f64,
}

/// Helper function to deal with tags in the data being [] when empty, but map otherwise
fn deserialize_tags<'de, D>(deserializer: D) -> Result<HashMap<String, u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    #[allow(dead_code)]
    enum TagsHelper {
        Map(HashMap<String, u64>),
        Array(Vec<de::IgnoredAny>), // Used to match `[]` and discard its content
    }

    match TagsHelper::deserialize(deserializer)? {
        TagsHelper::Map(map) => Ok(map),
        TagsHelper::Array(_) => Ok(HashMap::new()), // If it was `[]`, return empty map
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Encode, Decode)]
#[serde(untagged)] // This is the key: Serde tries each variant in order
pub enum StringOrU64 {
    String(String),
    U64(u64),
}

impl Default for StringOrU64 {
    fn default() -> Self {
        StringOrU64::U64(0)
    }
}
