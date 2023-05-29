use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataProfileEdit {
    pub str_persona_name: String,
    #[serde(rename = "strCustomURL")]
    pub str_custom_url: String,
    pub str_real_name: String,
    pub str_summary: String,
    pub str_avatar_hash: String,
    #[serde(rename = "LocationData")]
    pub location_data: LocationData,
    #[serde(rename = "ProfilePreferences")]
    pub profile_preferences: ProfilePreferences,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationData {
    pub loc_country: String,
    pub loc_country_code: String,
    pub loc_state: String,
    pub loc_state_code: String,
    pub loc_city: String,
    pub loc_city_code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePreferences {
    #[serde(rename = "hide_profile_awards")]
    pub hide_profile_awards: i64,
}
