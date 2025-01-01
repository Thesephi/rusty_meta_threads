use crate::shared::MetaMediaResponse;
use serde::Deserialize;

// https://developers.facebook.com/docs/threads/reply-management#a-thread-s-conversations
#[derive(Deserialize, Debug)]
pub struct MetaMedia {
    pub id: String,
    pub is_reply_owned_by_me: Option<bool>,
    pub username: Option<String>,
    pub text: Option<String>,
    pub timestamp: Option<String>,
    pub media_product_type: Option<String>, // THREADS | ...
    pub media_type: Option<String>,         // TEXT_POST | ...
    pub media_url: Option<String>,
    pub permalink: Option<String>,
    pub shortcode: Option<String>,
    pub has_replies: Option<bool>,
    // pub root_post: { id: "1234567890" },
    // pub replied_to: { id: "1234567890" },
    // pub is_reply: bool,
    // pub hide_status: String, // NOT_HUSHED | ...
}

pub async fn get_my_threads(
    limit: u32,
    token: &str,
) -> Result<MetaMediaResponse<MetaMedia>, reqwest::Error> {
    get_threads("me", "", "", &limit.to_string(), token).await
}

pub async fn get_threads(
    user_id: &str,
    since: &str,
    until: &str,
    limit: &str,
    token: &str,
) -> Result<MetaMediaResponse<MetaMedia>, reqwest::Error> {
    // @TODO get fields from method arguments
    let url = format!(
        "https://graph.threads.net/v1.0/{user_id}/threads\
        ?fields=id,media_product_type,media_type,media_url,permalink,\
        owner,username,text,timestamp,shortcode,thumbnail_url,children,\
        has_replies,is_quote_post\
        &since={since}&until={until}&limit={limit}&access_token={token}"
    );

    let res = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json::<MetaMediaResponse<MetaMedia>>()
        .await?;

    // @NOTE it may fail silently if we declare a field that's not included in
    // the actual response (e.g. `media_url` in the derive Deserialize ie
    // `MetaMediaResponse` in this case). A possible solution is to declare it
    // in the struct as optional ie `Option<media_url>`

    Ok(res)
}
