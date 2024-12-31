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

#[derive(Deserialize, Debug)]
pub struct MetaMediaResponse {
    pub data: Option<Vec<MetaMedia>>,
    pub paging: Option<Paging>,
    error: Option<ThreadsApiRespErrorPayload>,
}

#[derive(Deserialize, Debug)]
pub struct Paging {
    cursors: Cursors,
}

#[derive(Deserialize, Debug)]
pub struct Cursors {
    before: String,
    after: String,
}

#[derive(Deserialize, Debug)]
pub struct ThreadsApiRespErrorPayload {
    #[allow(dead_code)]
    pub message: String,
    code: u8,
    error_subcode: u16,
    fbtrace_id: String,
}
