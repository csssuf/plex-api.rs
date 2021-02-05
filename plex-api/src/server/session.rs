use crate::{InternalHttpApi, PlexApiError, Result, Server, SessionMediaContainer, SessionMediaContainerOuter};

const SESSIONS_URL: &str = "status/sessions";

impl Server {
    pub async fn get_sessions(&self) -> Result<SessionMediaContainer> {
        let response = self.get(SESSIONS_URL).await?;

        if response.status() == reqwest::StatusCode::OK {
            Ok(SessionMediaContainer::from(
                response.json::<SessionMediaContainerOuter>().await?,
            ))
        } else {
            Err(PlexApiError::UnexpectedApiResponse(response.text().await?))
        }
    }
}
