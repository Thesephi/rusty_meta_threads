use crate::shared::MetaMediaResponse;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SimpleMediaObject {
    pub id: String,
}

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
    pub root_post: Option<SimpleMediaObject>,
    pub replied_to: Option<SimpleMediaObject>,
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

pub async fn get_thread(
    thread_id: &str,
    fields: Option<&str>,
    token: &str,
) -> Result<MetaMedia, reqwest::Error> {
    let the_fields = fields.unwrap_or_else(|| "id,root_post,replied_to,media_product_type,media_type,media_url,permalink,owner,username,text,timestamp,shortcode,thumbnail_url,children,is_quote_post");

    let url = format!("https://graph.threads.net/v1.0/{thread_id}?fields={the_fields}");

    let res = reqwest::Client::new()
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<MetaMedia>()
        .await?;

    Ok(res)
}
