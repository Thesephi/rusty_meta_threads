# rusty_meta_threads

A community Rust SDK that helps interact with
[Meta Threads API](https://developers.facebook.com/docs/threads)

## Usage example

```
# configurations
# assuming these env values e.g. in `.env` file

THREADS_APP_ID=xxxxxxxxxxxxxxx
THREADS_APP_SECRET=xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
THREADS_APP_AUTH_SCOPE=threads_basic
THREADS_AUTH_CODE_REDIRECT_URI=<REDIRECT_URL>
```

```rs
use rusty_meta_threads::{auth, profiles};

//------------------------------
// CLIENT-SIDE

let login_url = auth::get_threads_login_url();
// now we can redirect the end-user to `login_url`,
// once the end-user logs in successfully, Threads will send
// a request to `<REDIRECT_URL>?code=<CODE>`, which we'll handle
// on the server-side

//------------------------------
// SERVER-SIDE

// inside the HTTP endpoint handler logic for e.g. my-app.domain/auth
let code =
    auth::get_code_from_redirect_uri("<REDIRECT_URL_WITH_CODE>");

// `code` can now be exchanged for a short-lived bearer token
let short_lived_token =
    rusty_meta_threads::get_short_lived_bearer_token(&code)
        .await
        .unwrap()
        .access_token;

// `short_lived_token` can be exchanged for
// a long-lived bearer token
let long_lived_token =
    auth::get_long_lived_bearer_token(&short_lived_token)
        .await
        .unwrap()
        .access_token;

// either `short_lived_token` or `long_lived_token` can be used
// to make API calls against Threads, for example:
let profile_info =
    profiles::get_profile_info(&long_lived_token)
        .await
        .unwrap();

// long-lived token can be refreshed
// as long as it doesn't expire yet
let refreshed_token =
    auth::refresh_long_lived_bearer_token(&long_lived_token)
        .await
        .unwrap()
        .access_token;
```

## Contributor notice

Please see [GOVERNANCE](./GOVERNANCE.md)
