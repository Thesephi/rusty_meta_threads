use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MetaMediaResponse<T> {
    pub data: Option<Vec<T>>,
    pub paging: Option<Paging>,
    pub error: Option<ThreadsApiRespErrorPayload>,
}

#[derive(Deserialize, Debug)]
pub struct Paging {
    cursors: Cursors,
    pub next: Option<String>,
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
    code: u32,
    error_subcode: Option<u32>,
    fbtrace_id: String,
}
