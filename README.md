# rusty_meta_threads

A library that helps interact with [Meta Threads API](https://developers.facebook.com/docs/threads)

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
use rusty_meta_threads;

//---------------
// client-side
let login_url = rusty_meta_threads::get_threads_login_url();
// now we can redirect the end-user to `login_url`
// once the end-user logs in successfully, Threads will send a request
// to <REDIRECT_URL>, which we'll handle on the server-side

//---------------
// server-side
let code = rusty_meta_threads::get_code_from_redirect_uri("<REDIRECT_URL>");
// `code` can now be exchanged for a short-lived bearer token

```

## Contributor notice

Please see [GOVERNANCE](./GOVERNANCE.md)
