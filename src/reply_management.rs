use crate::shared::MetaMediaResponse;

pub async fn get_conversations(
    media_id: &str,
    token: &str,
) -> Result<MetaMediaResponse, reqwest::Error> {
    // @TODO get fields from method arguments
    let reverse = false;
    let url = format!(
        "https://graph.threads.net/v1.0/{media_id}/conversation\
        ?fields=id,is_reply_owned_by_me,username,text,timestamp,\
        media_product_type,media_type,media_url,shortcode,thumbnail_url,\
        children,has_replies,root_post,replied_to,is_reply,hide_status\
        &reverse={reverse}\
        &access_token={token}"
    );

    let res = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json::<MetaMediaResponse>()
        .await?;

    Ok(res)
}
