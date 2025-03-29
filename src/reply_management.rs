pub use crate::retrieve_media::MetaMedia;
use crate::shared::MetaMediaResponse;

pub async fn get_conversations(
    media_id: &str,
    fields: Option<&str>,
    reverse: Option<bool>,
    token: &str,
) -> Result<MetaMediaResponse<MetaMedia>, reqwest::Error> {
    let the_fields = fields.unwrap_or_else(|| {
        "id,is_reply_owned_by_me,username,text,timestamp,\
        media_product_type,media_type,media_url,shortcode,thumbnail_url,\
        children,has_replies,root_post,replied_to,is_reply,hide_status"
    });
    let the_reverse = reverse.unwrap_or(false);
    let url = format!(
        "https://graph.threads.net/v1.0/{media_id}/conversation\
        ?fields={the_fields}\
        &reverse={the_reverse}"
    );

    let res = reqwest::Client::new()
        .get(&url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<MetaMediaResponse<MetaMedia>>()
        .await?;

    Ok(res)
}
