use crate::posts::publish_media_container;
use crate::retrieve_media::SimpleMediaObject;
use std::time::Duration;

pub async fn create_reply(
    reply_to_id: &str,
    text: Option<&str>,
    image_url: Option<&str>,
    video_url: Option<&str>,
    token: &str,
) -> Result<SimpleMediaObject, reqwest::Error> {
    let mut url = format!(
        "https://graph.threads.net/v1.0/me/threads\
        ?reply_to_id={reply_to_id}"
    );

    let mut publish_wait_time_ms = 300;
    let mut media_type = "TEXT";
    if let Some(text) = text {
        url.push_str(format!("&text={text}").as_str());
    }
    if let Some(image_url) = image_url {
        url.push_str(format!("&image_url={image_url}").as_str());
        media_type = "IMAGE";
        publish_wait_time_ms = 3000;
    }
    if let Some(video_url) = video_url {
        url.push_str(format!("&video_url={video_url}").as_str());
        media_type = "VIDEO";
        publish_wait_time_ms = 30000;
    }
    url.push_str(format!("&media_type={media_type}").as_str());

    let media_container = reqwest::Client::new()
        .post(&url)
        .bearer_auth(token)
        .send()
        .await? // @TODO don't silently fail on expired token (see profiles.rs example)
        .json::<SimpleMediaObject>()
        .await?;

    // ideally we proceed as long as we have `id` in the media_container, or poll until we have it
    // https://developers.facebook.com/docs/threads/troubleshooting#publishing-does-not-return-a-media-id
    // but for now it's alright to stick with some hardcoded wait time
    tokio::time::sleep(Duration::from_millis(publish_wait_time_ms)).await;

    assert_eq!(media_container.id.is_some(), true);

    let res = publish_media_container(media_container.id.unwrap().as_str(), token).await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_dot_env;
    use log::debug;
    use urlencoding::encode;

    #[tokio::test]
    async fn test_create_reply() {
        let should_log_verbose = true;
        let _ = env_logger::builder()
            .is_test(!should_log_verbose)
            .try_init();

        let env = read_dot_env();
        let token = env.get("ACCESS_TOKEN").unwrap();

        let reply_to_id = "17961951074882947";
        let text = encode("you see me rollin' 🥁");
        let image_url = "https://i.imgur.com/Cj33AKk.png";
        let res = create_reply(reply_to_id, Some(&*text), Some(image_url), None, &token).await;

        debug!("{:?}", res);
        assert_eq!(true, res.is_ok());
    }
}
