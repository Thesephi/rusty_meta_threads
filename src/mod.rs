use log::{debug, warn};
use serde::Deserialize;
use url::Url;
use urlencoding::encode;
mod utils;

// @TODO contemplate if we should make env vars static

#[derive(Deserialize, Debug)]
struct ThreadsApiRespErrorPayload {
    #[allow(dead_code)]
    message: String,
    // code: u8,
    // error_subcode: u16,
    // fbtrace_id: String,
}

#[derive(Deserialize, Debug)]
pub struct SimpleThreadsShortLivedTokenResponse {
    pub access_token: Option<String>,
    pub user_id: Option<u64>,
    error: Option<ThreadsApiRespErrorPayload>,
}

#[derive(Deserialize, Debug)]
pub struct SimpleThreadsLongLivedTokenResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub expires_in: Option<u32>,
    #[allow(dead_code)]
    error: Option<ThreadsApiRespErrorPayload>,
}

#[derive(Deserialize, Debug)]
pub struct ThreadsUserProfile {
    pub id: Option<String>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub threads_profile_picture_url: Option<String>,
    pub threads_biography: Option<String>,
    error: Option<ThreadsApiRespErrorPayload>,
}

pub fn get_threads_login_url() -> String {
    get_threads_login_url_for_state("rusty_meta_threads")
}

// @TODO check whether it's a better practice to return `Cow<'_, str>`
pub fn get_threads_login_url_for_state(app_state: &str) -> String {
    let env = utils::read_dot_env();

    let app_id = env
        .get("THREADS_APP_ID")
        .expect("missing env: THREADS_APP_ID");

    let redirect_uri = encode(
        env.get("THREADS_AUTH_CODE_REDIRECT_URI")
            .expect("missing env: THREADS_AUTH_CODE_REDIRECT_URI"),
    );

    let app_scope = match env.get("THREADS_APP_AUTH_SCOPE") {
        Some(val) => val,
        None => {
            warn!("missing env: THREADS_APP_AUTH_SCOPE, defaulting to 'threads_basic'");
            "threads_basic"
        }
    };

    format!(
        "https://threads.net/oauth/authorize\
    ?client_id={app_id}\
    &redirect_uri={redirect_uri}\
    &scope={app_scope}\
    &response_type=code\
    &state={app_state}"
    )
}

pub fn get_code_from_redirect_uri(url: &str) -> String {
    let mut input = String::from("http://localhost"); // value not important
    input.push_str(url);
    let url = Url::parse(&input).unwrap();
    for (key, val) in url.query_pairs() {
        if key.eq_ignore_ascii_case("code") {
            return String::from(val);
        };
    }
    warn!("no 'code' query recognized from the input URL");
    String::from("")
}

// @TODO document that this expires in 1 hour
pub async fn get_short_lived_bearer_token(
    code: &str,
) -> Result<SimpleThreadsShortLivedTokenResponse, reqwest::Error> {
    let env = utils::read_dot_env();

    let app_id = env
        .get("THREADS_APP_ID")
        .expect("missing env: THREADS_APP_ID");

    let app_secret = env
        .get("THREADS_APP_SECRET")
        .expect("missing env: THREADS_APP_SECRET");

    let redirect_uri = encode(
        env.get("THREADS_AUTH_CODE_REDIRECT_URI")
            .expect("missing env: THREADS_AUTH_CODE_REDIRECT_URI"),
    );

    let url = format!(
        "https://graph.threads.net/oauth/access_token\
        ?client_id={app_id}\
        &client_secret={app_secret}\
        &code={code}\
        &grant_type=authorization_code\
        &redirect_uri={redirect_uri}"
    );

    let res = reqwest::Client::new()
        .post(url)
        .send()
        .await?
        .json::<SimpleThreadsShortLivedTokenResponse>()
        .await?;

    match res.access_token {
        Some(_) => Ok(res),
        None => {
            debug!("failed to retrieve short-lived token: {:#?}", res.error);
            // @TODO consider using Err instead of Ok
            Ok(SimpleThreadsShortLivedTokenResponse {
                access_token: None,
                user_id: None,
                error: res.error,
            })
        }
    }
}

// @TODO document that this expires in 60 days
pub async fn get_long_lived_bearer_token(
    short_lived_token: &str,
) -> Result<SimpleThreadsLongLivedTokenResponse, reqwest::Error> {
    let env = utils::read_dot_env();

    let app_secret = env
        .get("THREADS_APP_SECRET")
        .expect("missing env: THREADS_APP_SECRET");

    let url = format!(
        "https://graph.threads.net/access_token\
        ?grant_type=th_exchange_token\
        &client_secret={app_secret}\
        &access_token={short_lived_token}"
    );

    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<SimpleThreadsLongLivedTokenResponse>()
        .await?;

    match res.access_token {
        Some(_) => Ok(res),
        None => {
            debug!("failed to retrieve long-lived token: {:#?}", res);
            // @TODO consider using Err instead of Ok
            Ok(SimpleThreadsLongLivedTokenResponse {
                access_token: None,
                token_type: None,
                expires_in: None,
                error: None,
            })
        }
    }
}

pub async fn refresh_long_lived_bearer_token(
    long_lived_token: &str,
) -> Result<SimpleThreadsLongLivedTokenResponse, reqwest::Error> {
    let url = format!(
        "https://graph.threads.net/refresh_access_token\
        ?grant_type=th_refresh_token\
        &access_token={long_lived_token}"
    );

    let res = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .json::<SimpleThreadsLongLivedTokenResponse>()
        .await?;

    match res.access_token {
        Some(_) => Ok(res),
        None => {
            debug!("failed to refresh long-lived token: {:#?}", res);
            // @TODO consider using Err instead of Ok
            Ok(SimpleThreadsLongLivedTokenResponse {
                access_token: None,
                token_type: None,
                expires_in: None,
                error: None,
            })
        }
    }
}

// @TODO have fields as fn arguments instead of hardcoding
pub async fn get_profile_info(bearer_token: &str) -> Result<ThreadsUserProfile, reqwest::Error> {
    let url = "https://graph.threads.net/me?fields=id%2Cusername%2Cname%2Cthreads_profile_picture_url%2Cthreads_biography";

    let res = reqwest::Client::new()
        .get(url)
        .bearer_auth(bearer_token)
        .send()
        .await?
        .json::<ThreadsUserProfile>()
        .await?;

    match res.error {
        Some(error) => {
            debug!("failed to retrieve Threads user profile: {:#?}", error);
            // @TODO consider using Err instead of Ok
            Ok(ThreadsUserProfile {
                id: None,
                username: None,
                name: None,
                threads_biography: None,
                threads_profile_picture_url: None,
                error: Some(error),
            })
        }
        None => Ok(res),
    }
}
