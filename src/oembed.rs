use log::debug;
use serde::Deserialize;
use urlencoding::encode;

#[derive(Deserialize, Debug)]
pub struct OembedResponse {
    pub version: String,
    pub provider_name: String,
    pub provider_url: String,
    pub width: u64,
    pub html: String,
}

pub async fn get_oembed_html(
    post_url: &str,
    token: &str,
) -> Result<OembedResponse, reqwest::Error> {
    let the_post_url = encode(post_url);
    let url = format!(
        "https://graph.threads.net/v1.0/oembed?url={the_post_url}\
        &access_token={token}"
    );

    debug!("requesting oembed for: {:?}", &url);

    let res = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json::<OembedResponse>()
        .await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_dot_env;
    use log::debug;

    #[tokio::test]
    async fn test_get_oembed_html() {
        let should_log_verbose = true;
        let _ = env_logger::builder()
            .is_test(!should_log_verbose)
            .try_init();

        let env = read_dot_env();
        let token = env.get("ACCESS_TOKEN").unwrap();

        let res =
            get_oembed_html("https://www.threads.net/@dkode___/post/DHrAOfItbTe", token).await;

        debug!("oembed response fetched: {:?}", res);

        assert_eq!(true, res.is_ok());
        assert_eq!(res.unwrap().html, "");
    }
}
