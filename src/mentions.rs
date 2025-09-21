pub use crate::retrieve_media::MetaMedia;
use crate::shared::MetaMediaResponse;

pub async fn get_mentions(
    user_id: &str,
    fields: Option<&str>,
    token: &str,
) -> Result<MetaMediaResponse<MetaMedia>, reqwest::Error> {
    let the_fields = fields.unwrap_or_else(|| "id,username,text,media_url,root_post,replied_to");

    let url = format!(
        "https://graph.threads.net/{user_id}/mentions\
        ?fields={the_fields}",
    );

    let res = reqwest::Client::new()
        .get(url)
        .bearer_auth(token)
        .send()
        .await? // @TODO don't silently fail on expired token (see profiles.rs example)
        .json::<MetaMediaResponse<MetaMedia>>()
        .await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_dot_env;
    use log::debug;
    use std::any::{Any, TypeId};

    fn is_meta_media_vec<T: Any>(_val: &T) -> bool {
        TypeId::of::<T>() == TypeId::of::<Vec<MetaMedia>>()
    }

    #[tokio::test]
    async fn test_get_mentions() {
        let should_log_verbose = true;
        let _ = env_logger::builder()
            .is_test(!should_log_verbose)
            .try_init();

        let env = read_dot_env();
        let token = env.get("ACCESS_TOKEN").unwrap();

        let res = get_mentions("me", None, token).await;
        match res {
            Ok(val) => match val.data {
                Some(dat) => {
                    debug!("mentions fetched: {:?}", dat);
                    assert_eq!(is_meta_media_vec(&dat), true);
                }
                None => panic!("unexpected result"),
            },
            Err(e) => panic!("unexpected result: {}", e),
        }
    }
}
