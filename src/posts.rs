use crate::retrieve_media::SimpleMediaObject;

pub async fn publish_media_container(
    media_container_id: &str,
    token: &str,
) -> Result<SimpleMediaObject, reqwest::Error> {
    let url = format!(
        "https://graph.threads.net/v1.0/me/threads_publish\
        ?creation_id={media_container_id}\
        &access_token={token}"
    );

    let res = reqwest::Client::new()
        .post(&url)
        .send()
        .await?
        .json::<SimpleMediaObject>()
        .await?;

    Ok(res)
}
