//! Set of constants used to upload to the service.
// To those reading the source: the `SHORTEN_URL` and `UPLOAD_URL` constants can
// not be used to format due to macro rule restrictions, but are here for
// completion.

/// The maximum number of files that may be uploaded in one requests.
pub const MAX_FILES: usize = 3;
/// The URL to POST to, to request shortened URLs.
pub const SHORTEN_URL: &'static str = "https://api.awau.moe/shorten/polr?action=shorten&url={}&key={}";
/// The URL to POST to, to upload files.
pub const UPLOAD_URL: &'static str = "https://api.awau.moe/upload/pomf?key={}";
/// The user agent to send along with requests.
pub const USER_AGENT: &'static str = concat!(
    "WhatsThisClient (https://github.com/whats-this/owo.rs, ",
    env!("CARGO_PKG_VERSION"),
    ")",
);
