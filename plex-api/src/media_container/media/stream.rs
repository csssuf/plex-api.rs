use std::convert::Infallible;
use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde_aux::field_attributes::{
    deserialize_number_from_string,
    deserialize_option_number_from_string,
};
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum MediaStreamType {
    Unknown = 0,
    Video = 1,
    Audio = 2,
    Subtitles = 3,
    Lyrics = 4,
}

impl Default for MediaStreamType {
    fn default() -> MediaStreamType {
        MediaStreamType::Unknown
    }
}

impl FromStr for MediaStreamType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<MediaStreamType, Self::Err> {
        Ok(u8::from_str(s)
            .map(|v| match v {
                1 => MediaStreamType::Video,
                2 => MediaStreamType::Audio,
                3 => MediaStreamType::Subtitles,
                4 => MediaStreamType::Lyrics,
                _ => MediaStreamType::Unknown,
            })
            .unwrap_or(MediaStreamType::Unknown))
    }
}

#[derive(Debug, Clone)]
pub enum MediaStream {
    Video(VideoStream),
    Audio(AudioStream),
    Subtitles(SubtitlesStream),
    Lyrics(LyricsStream),
}

#[derive(Debug, Default, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(default, rename_all = "camelCase")]
struct MediaStreamStruct {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    stream_type: MediaStreamType,
    default: Option<bool>,
    #[serde(deserialize_with = "crate::serde_helpers::option_bool_from_anything")]
    selected: Option<bool>,
    codec: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    index: Option<u8>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    bitrate: Option<u32>,
    chroma_subsampling: Option<String>,
    chroma_location: Option<String>,
    closed_captions: Option<String>,
    coded_height: Option<String>,
    coded_width: Option<String>,
    color_primaries: Option<String>,
    color_space: Option<String>,
    color_range: Option<String>,
    color_trc: Option<String>,
    frame_rate: Option<f32>,
    height: Option<u16>,
    width: Option<u16>,
    level: Option<u16>,
    profile: Option<String>,
    ref_frames: Option<u64>,
    display_title: String,
    has_scaling_matrix: Option<bool>,
    scan_type: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    bit_depth: Option<u16>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    sampling_rate: Option<u32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    channels: Option<u8>,
    audio_channel_layout: Option<String>,
    key: Option<String>,
    title: Option<String>,
    language: Option<String>,
    language_code: Option<String>,
    #[serde(deserialize_with = "crate::serde_helpers::option_bool_from_anything")]
    embedded_in_video: Option<bool>,
    extended_display_title: Option<String>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    album_gain: Option<f32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    album_peak: Option<f32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    album_range: Option<f32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    gain: Option<f32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    loudness: Option<f32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    lra: Option<f32>,
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    peak: Option<f32>,
    format: Option<String>,
    provider: Option<String>,
}

macro_rules! media_stream_enum {
    (pub struct $name:ident {
        $($field_name:ident: $field_type:ty,)+
    }) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub id: u32,
            pub stream_type: MediaStreamType,
            pub codec: Option<String>,
            pub index: Option<u8>,
            pub display_title: String,
            pub $($field_name: $field_type,)+
        }

        impl From<MediaStreamStruct> for $name {
            fn from(stream: MediaStreamStruct) -> Self {
                $name {
                    id: stream.id,
                    stream_type: stream.stream_type,
                    codec: stream.codec,
                    index: stream.index,
                    display_title: stream.display_title,
                    $($field_name: <$field_type>::convert(stringify!($field_name), stream.$field_name)),+
                }
            }
        }
    }
}

trait InternalTypesConverter<T>: Sized {
    fn convert(field: &str, _: T) -> Self;
}

impl<T> InternalTypesConverter<Option<T>> for Option<T> {
    fn convert(_field: &str, input: Option<T>) -> Self {
        input
    }
}

impl<T: Default> InternalTypesConverter<Option<T>> for T {
    fn convert(field: &str, input: Option<T>) -> Self {
        if let Some(value) = input {
            value
        } else {
            warn!("Error while processing field {}", field);
            Default::default()
        }
    }
}

media_stream_enum! {
    pub struct VideoStream {
        default: bool,
        bitrate: u32,
        chroma_subsampling: String,
        chroma_location: String,
        closed_captions: String,
        coded_height: String,
        coded_width: String,
        color_primaries: String,
        color_space: String,
        color_range: String,
        color_trc: String,
        frame_rate: f32,
        height: u16,
        width: u16,
        level: u16,
        profile: String,
        ref_frames: u64,
        has_scaling_matrix: bool,
        scan_type: String,
        bit_depth: Option<u16>,
    }
}

media_stream_enum! {
    pub struct AudioStream {
        default: Option<bool>,
        selected: bool,
        bitrate: u32,
        profile: String,
        sampling_rate: u32,
        channels: u8,
        audio_channel_layout: String,
        album_gain: Option<f32>,
        album_peak: Option<f32>,
        album_range: Option<f32>,
        bit_depth: Option<u16>,
        extended_display_title: Option<String>,
        gain: Option<f32>,
        loudness: Option<f32>,
        lra: Option<f32>,
        peak: Option<f32>,
    }
}

media_stream_enum! {
    pub struct SubtitlesStream {
        selected: bool,
        bitrate: Option<u32>,
        embedded_in_video: bool,
    }
}

media_stream_enum! {
    pub struct LyricsStream {
        extended_display_title: Option<String>,
        format: Option<String>,
        provider: Option<String>,
    }
}

impl MediaStream {
    fn new(stream: MediaStreamStruct) -> Self {
        match stream.stream_type {
            MediaStreamType::Unknown => unimplemented!(),
            MediaStreamType::Video => MediaStream::Video(VideoStream::from(stream)),
            MediaStreamType::Audio => MediaStream::Audio(AudioStream::from(stream)),
            MediaStreamType::Subtitles => MediaStream::Subtitles(SubtitlesStream::from(stream)),
            MediaStreamType::Lyrics => MediaStream::Lyrics(LyricsStream::from(stream)),
        }
    }
}

impl<'de> Deserialize<'de> for MediaStream {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let stream = MediaStreamStruct::deserialize(d)?;
        Ok(MediaStream::new(stream))
    }
}
