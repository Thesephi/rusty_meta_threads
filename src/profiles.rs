use crate::shared::ThreadsApiRespErrorPayload;
use log::debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ThreadsUserProfile {
    pub id: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub threads_profile_picture_url: Option<String>,
    pub threads_biography: Option<String>,
    pub error: Option<ThreadsApiRespErrorPayload>,
}

pub async fn get_profile_info(
    fields: Option<&str>,
    bearer_token: &str,
) -> Result<ThreadsUserProfile, reqwest::Error> {
    let the_fields =
        fields.unwrap_or_else(|| "id,username,name,threads_profile_picture_url,threads_biography");
    let url = format!(
        "https://graph.threads.net/me\
        ?fields={the_fields}"
    );

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
                id: None,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_dot_env;
    #[tokio::test]
    async fn test_get_profile_info() {
        let should_log_verbose = true;
        let _ = env_logger::builder()
            .is_test(!should_log_verbose)
            .try_init();

        let env = read_dot_env();
        let token = env.get("ACCESS_TOKEN").unwrap();

        let res = get_profile_info(None, token).await;

        /*
         * @TODO test against invalid access_token, which results in this response
         * {
         *     message: "Error validating access token: The session has been invalidated because the user changed their password or Facebook has changed the session for security reasons.",
         *     code: 190,
         *     error_subcode: None,
         *     fbtrace_id: Some("A6p8XCWpTMHwh06sQG-Jv04"),
         * }
         */

        debug!("profile fetched {:?}", res);

        assert_eq!(true, res.is_ok());
    }
}
