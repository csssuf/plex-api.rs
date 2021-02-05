use crate::MediaMetadata;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub struct SessionUser {
    #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
    id: u32,
    thumb: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub address: String,
    pub device: String,
    pub machine_identifier: String,
    pub model: Option<String>,
    pub platform: String,
    pub platform_version: String,
    pub product: String,
    pub remote_public_address: String,
    pub state: String,
    pub title: String,
    pub vendor: Option<String>,
    pub version: String,
    pub local: bool,
    pub relayed: bool,
    pub secure: bool,
    #[serde(rename = "userID")]
    user_id: u32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SessionMetadata {
    #[serde(flatten)]
    pub metadata: MediaMetadata,
    #[serde(rename = "User")]
    pub user: SessionUser,
    #[serde(rename = "Player")]
    pub player: Player,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
#[serde(rename_all = "camelCase")]
pub struct SessionMediaContainer {
    size: u64,
    #[serde(default, rename = "Metadata")]
    pub metadata: Vec<SessionMetadata>,
}


#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(all(test, feature = "test_new_attributes"), serde(deny_unknown_fields))]
pub(crate) struct SessionMediaContainerOuter {
    #[serde(rename = "MediaContainer")]
    media_container: SessionMediaContainer,
}

impl From<SessionMediaContainerOuter> for SessionMediaContainer {
    fn from(mc: SessionMediaContainerOuter) -> Self {
        mc.media_container
    }
}
