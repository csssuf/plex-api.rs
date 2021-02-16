use serde_aux::field_attributes::deserialize_number_from_string;

use crate::{MediaStream, MediaType};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaPart {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u32,
    pub key: String,
    #[serde(deserialize_with = "crate::serde_helpers::duration_from_seconds")]
    pub duration: chrono::Duration,
    pub file: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub size: u64,
    pub container: String,
    pub indexes: Option<String>,
    pub audio_profile: Option<String>,
    pub video_profile: Option<String>,
    #[serde(default, rename = "Stream")]
    pub streams: Vec<MediaStream>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaTag {
    id: Option<u32>,
    tag: String,
    filter: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Media {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub id: u32,
    #[serde(deserialize_with = "crate::serde_helpers::duration_from_seconds")]
    pub duration: chrono::Duration,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub bitrate: u32,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub aspect_ratio: Option<f32>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub audio_channels: u8,
    pub audio_codec: String,
    pub video_codec: Option<String>,
    pub video_resolution: Option<String>,
    pub container: String,
    pub video_frame_rate: Option<String>,
    pub audio_profile: Option<String>,
    pub video_profile: Option<String>,
    #[serde(rename = "Part")]
    pub parts: Option<Vec<MediaPart>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    pub allow_sync: Option<bool>,
    #[serde(rename = "librarySectionID", deserialize_with = "deserialize_number_from_string")]
    pub library_section_id: u32,
    pub library_section_title: String,
    #[serde(rename = "librarySectionUUID")]
    pub library_section_uuid: Option<Uuid>,
    pub rating_key: String,
    pub key: String,
    pub skip_parent: Option<bool>,
    pub parent_rating_key: Option<String>,
    pub grandparent_rating_key: Option<String>,
    pub guid: Option<String>,
    pub parent_guid: Option<String>,
    pub grandparent_guid: Option<String>,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub title: String,
    pub grandparent_key: Option<String>,
    pub parent_key: Option<String>,
    pub library_section_key: Option<String>,
    pub grandparent_title: Option<String>,
    pub parent_title: Option<String>,
    pub content_rating: Option<String>,
    pub summary: String,
    #[serde(deserialize_with = "crate::serde_helpers::option_int_from_string")]
    pub index: Option<u32>,
    #[serde(deserialize_with = "crate::serde_helpers::option_int_from_string")]
    pub parent_index: Option<u32>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_int_from_string"
    )]
    pub year: Option<u32>,
    pub thumb: String,
    pub art: Option<String>,
    pub parent_thumb: Option<String>,
    pub grandparent_thumb: Option<String>,
    pub grandparent_art: Option<String>,
    pub grandparent_theme: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_duration_from_seconds"
    )]
    pub duration: Option<chrono::Duration>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_date_from_iso"
    )]
    pub originally_available_at: Option<chrono::Date<chrono::Utc>>,
    #[serde(deserialize_with = "crate::serde_helpers::datetime_from_seconds_string")]
    pub added_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, deserialize_with = "crate::serde_helpers::option_datetime_from_seconds_string")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, rename = "Media")]
    pub media: Vec<Media>,
    #[serde(rename = "Genre")]
    pub genre: Option<Vec<MediaTag>>,
    #[serde(rename = "Director")]
    pub director: Option<Vec<MediaTag>>,
    #[serde(rename = "Writer")]
    pub writer: Option<Vec<MediaTag>>,
    #[serde(rename = "Country")]
    pub country: Option<Vec<MediaTag>>,
    #[serde(rename = "Role")]
    pub role: Option<Vec<MediaTag>>,
    pub leaf_count: Option<u32>,
    pub viewed_leaf_count: Option<u32>,
    pub loudness_analysis_version: Option<String>,
    pub deep_analysis_version: Option<String>,
    pub studio: Option<String>,
    pub rating: Option<f32>,
    pub tagline: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_bool_from_anything"
    )]
    pub has_premium_primary_extra: Option<bool>,
    pub primary_extra_key: Option<String>,
    pub rating_image: Option<String>,
    pub parent_summary: Option<String>,
    pub parent_theme: Option<String>,
    #[serde(default)]
    pub original_title: Option<String>,
}
