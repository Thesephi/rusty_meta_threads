## [0.6.0] - 2025-03-17

### Added

- `get_thread()` lib fn
- `get_mentions()` lib fn now also retrieves `root_post` and `replied_to` fields by default
- explicit license files included in repo

## [0.5.0] - 2025-03-15

### Changed

- rust edition to `2024`

### Added

- `get_mentions()` lib fn
- debug logging for `read_dot_env()`

### Fixed

- several `reqwest` response decoding failures were fixed
- `read_dot_env()` should work with latest rust version (handling both lines with and without comments)

## [0.4.0] - 2025-01-01

### Added

- Structs are organized in favour of generated docs e.g. `ThreadsUserProfile` and `MetaMedia`
- updated example code in README

### Changed

- `id` field becomes mandatory in Structs `ThreadsUserProfile` and `MetaMedia`

## [0.3.0] - 2025-01-01

### Changed

- lib functions are exported into namespaces following public Meta Threads API categories: https://developers.facebook.com/docs/threads

## [0.2.0] - 2024-12-31

### Added

- `get_my_threads()` lib fn
- `get_threads()` lib fn
- `get_conversations()` lib fn

### Changed

- README example wording

## [0.1.4] - 2024-12-31

### Changed

- README wordings & format (ie to fit crates.io page width)
- `get_code_from_redirect_uri()` now issues a WARN level log if it finds no
  `code` at all
- slight enhancements owing to the removal of unnecessary `to_string()` calls

### Fixed

- fixed a bug in `refresh_long_lived_bearer_token()` where some extra space
  characters might prevent the function from behaving correctly

## [0.1.3] - 2024-12-30

### Changed

- direct env vars (CLI arguments) take precedence over values in `.env` file
- failure to read `.env` file or missing required envs now results in useful
  error messages (assuming a
  [compatible log implementation](https://github.com/rust-lang/log?tab=readme-ov-file#in-executables))
- license change (following a rust community convention)

## [0.1.2] - 2024-12-30

### Added

- `get_code_from_redirect_uri` lib fn
- `get_short_lived_bearer_token` lib fn
- `get_long_lived_bearer_token` lib fn
- `refresh_long_lived_bearer_token` lib fn
- `get_profile_info` lib fn

### Changed

- updated crate description
- updated README

## [0.1.1] - 2024-12-29

### Added

- `get_threads_login_url` lib fn
- `get_threads_login_url_for_state` lib fn
