use serde_aux::field_attributes::deserialize_number_from_string;

use crate::{MediaStream, MediaType};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaPart {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    key: String,
    #[serde(deserialize_with = "crate::serde_helpers::duration_from_seconds")]
    duration: chrono::Duration,
    file: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    size: u64,
    container: String,
    indexes: Option<String>,
    audio_profile: Option<String>,
    video_profile: Option<String>,
    #[serde(default, rename = "Stream")]
    streams: Vec<MediaStream>,
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
    id: u32,
    #[serde(deserialize_with = "crate::serde_helpers::duration_from_seconds")]
    duration: chrono::Duration,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    bitrate: u32,
    width: Option<u16>,
    height: Option<u16>,
    aspect_ratio: Option<f32>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    audio_channels: u8,
    audio_codec: String,
    video_codec: Option<String>,
    video_resolution: Option<String>,
    container: String,
    video_frame_rate: Option<String>,
    audio_profile: Option<String>,
    video_profile: Option<String>,
    #[serde(rename = "Part")]
    parts: Option<Vec<MediaPart>>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct MediaMetadata {
    allow_sync: Option<bool>,
    #[serde(rename = "librarySectionID", deserialize_with = "deserialize_number_from_string")]
    library_section_id: u32,
    library_section_title: String,
    #[serde(rename = "librarySectionUUID")]
    library_section_uuid: Option<Uuid>,
    rating_key: String,
    key: String,
    skip_parent: Option<bool>,
    parent_rating_key: Option<String>,
    grandparent_rating_key: Option<String>,
    guid: Option<String>,
    parent_guid: Option<String>,
    grandparent_guid: Option<String>,
    #[serde(rename = "type")]
    media_type: MediaType,
    title: String,
    grandparent_key: Option<String>,
    parent_key: Option<String>,
    library_section_key: Option<String>,
    grandparent_title: Option<String>,
    parent_title: Option<String>,
    content_rating: Option<String>,
    summary: String,
    #[serde(deserialize_with = "crate::serde_helpers::option_int_from_string")]
    index: Option<u32>,
    #[serde(deserialize_with = "crate::serde_helpers::option_int_from_string")]
    parent_index: Option<u32>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_int_from_string"
    )]
    year: Option<u32>,
    thumb: String,
    art: Option<String>,
    parent_thumb: Option<String>,
    grandparent_thumb: Option<String>,
    grandparent_art: Option<String>,
    grandparent_theme: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_duration_from_seconds"
    )]
    duration: Option<chrono::Duration>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_date_from_iso"
    )]
    originally_available_at: Option<chrono::Date<chrono::Utc>>,
    #[serde(deserialize_with = "crate::serde_helpers::datetime_from_seconds_string")]
    added_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, deserialize_with = "crate::serde_helpers::option_datetime_from_seconds_string")]
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, rename = "Media")]
    media: Vec<Media>,
    #[serde(rename = "Genre")]
    genre: Option<Vec<MediaTag>>,
    #[serde(rename = "Director")]
    director: Option<Vec<MediaTag>>,
    #[serde(rename = "Writer")]
    writer: Option<Vec<MediaTag>>,
    #[serde(rename = "Country")]
    country: Option<Vec<MediaTag>>,
    #[serde(rename = "Role")]
    role: Option<Vec<MediaTag>>,
    leaf_count: Option<u32>,
    viewed_leaf_count: Option<u32>,
    loudness_analysis_version: Option<String>,
    deep_analysis_version: Option<String>,
    studio: Option<String>,
    rating: Option<f32>,
    tagline: Option<String>,
    #[serde(
        default,
        deserialize_with = "crate::serde_helpers::option_bool_from_anything"
    )]
    has_premium_primary_extra: Option<bool>,
    primary_extra_key: Option<String>,
    rating_image: Option<String>,
    parent_summary: Option<String>,
    parent_theme: Option<String>,
}
