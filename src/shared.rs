use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MetaMediaResponse<T> {
    pub data: Option<Vec<T>>,
    pub paging: Option<Paging>,
    pub error: Option<ThreadsApiRespErrorPayload>,
}

#[derive(Deserialize, Debug)]
pub struct Paging {
    #[allow(dead_code)]
    cursors: Cursors,
    pub next: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Cursors {
    #[allow(dead_code)]
    before: String,
    #[allow(dead_code)]
    after: String,
}

#[derive(Deserialize, Debug)]
pub struct ThreadsApiRespErrorPayload {
    #[allow(dead_code)]
    pub message: String,
    #[allow(dead_code)]
    code: u32,
    #[allow(dead_code)]
    error_subcode: Option<u32>,
    #[allow(dead_code)]
    fbtrace_id: String,
}
