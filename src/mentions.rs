pub use crate::retrieve_media::MetaMedia;
use crate::shared::MetaMediaResponse;

pub async fn get_mentions(
    user_id: &str,
    fields: Option<&str>,
    token: &str,
) -> Result<MetaMediaResponse<MetaMedia>, reqwest::Error> {
    let the_fields = if let Some(f) = fields {
        f
    } else {
        "id,username,text,media_url,root_post,replied_to"
    };

    let url = format!(
        "https://graph.threads.net/{user_id}/mentions\
        ?fields={the_fields}",
    );

    let res = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await?
        .json::<MetaMediaResponse<MetaMedia>>()
        .await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let res = get_mentions("foo", None, "bar").await;
        match res {
            Ok(val) => match val.data {
                Some(dat) => assert_eq!(dat[0].id, "foo"),
                None => panic!("unexpected result"),
            },
            Err(e) => panic!("unexpected result: {}", e),
        }
    }
}
