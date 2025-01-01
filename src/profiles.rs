use crate::shared::ThreadsApiRespErrorPayload;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ThreadsUserProfile {
    pub id: String,
    pub username: Option<String>,
    pub name: Option<String>,
    pub threads_profile_picture_url: Option<String>,
    pub threads_biography: Option<String>,
    pub error: Option<ThreadsApiRespErrorPayload>,
}

// @TODO have fields as fn arguments instead of hardcoding
pub async fn get_profile_info(bearer_token: &str) -> Result<ThreadsUserProfile, reqwest::Error> {
    let url = "https://graph.threads.net/me\
        ?fields=id%2Cusername%2Cname%2C\
        threads_profile_picture_url%2Cthreads_biography";

    let res = reqwest::Client::new()
        .get(url)
        .bearer_auth(bearer_token)
        .send()
        .await?
        .json::<ThreadsUserProfile>()
        .await?;

    match res.error {
        Some(error) => {
            debug!("failed to retrieve Threads user profile: {:#?}", error);
            // @TODO consider using Err instead of Ok
            Ok(ThreadsUserProfile {
                id: String::from(""),
                username: None,
                name: None,
                threads_biography: None,
                threads_profile_picture_url: None,
                error: Some(error),
            })
        }
        None => Ok(res),
    }
}
