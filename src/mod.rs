use url::Url;
use urlencoding::encode;
mod utils;

pub fn get_threads_login_url() -> String {
    get_threads_login_url_for_state("rusty_threads")
}

pub fn get_threads_login_url_for_state(app_state: &str) -> String {
    let env = utils::read_dot_env();
    let app_id = env.get("THREADS_APP_ID").unwrap();
    let redirect_uri = encode(env.get("THREADS_AUTH_CODE_REDIRECT_URI").unwrap());
    let app_scope = match env.get("THREADS_APP_AUTH_SCOPE") {
        Some(val) => val,
        None => "threads_basic",
    };

    let url_tpl = format!(
        "https://threads.net/oauth/authorize\
    ?client_id={app_id}\
    &redirect_uri={redirect_uri}\
    &scope={app_scope}\
    &response_type=code\
    &state={app_state}"
    );
    url_tpl.to_string()
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
    return String::from("");
}
