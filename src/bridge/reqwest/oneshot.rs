//! A collection of oneshot functions for performing requests over the API.
//!
//! These are functions that create a oneshot [`OwoReqwestClient`]. Use at your
//! own expense. Oneshots are more expensive with repeated use.
//!
//! [`OwoReqwestClient`]: ../struct.OwoClient.html

use model::{FileUploadResponse, UploadedFile};
use super::{OwoClient, OwoRequester};
use ::Result;

/// Uploads a single file via the service.
///
/// Refer to [`OwoRequester::upload_file`] for more information.
///
/// # Errors
///
/// Returns [`Error::Reqwest`] if building the request fails.
///
/// [`Error::Reqwest`]: ../../../enum.Error.html#variant.Reqwest
/// [`OwoReqwestRequester::upload_file`]: ../trait.OwoRequester.html#tymethod.upload_file
#[inline]
pub fn upload_file(file: Vec<u8>) -> Result<FileUploadResponse> {
    OwoClient::new(key)?.upload_file(file)
}

/// Uploads multiple files via the service.
///
/// See [`OwoReqwestRequester::upload_files`] for more information.
///
/// # Errors
///
/// Returns [`Error::Reqwest`] if building the request fails.
///
/// [`Error::Reqwest`]: ../../../enum.Error.html#variant.Reqwest
/// [`OwoReqwestRequester::upload_files`]: ../trait.OwoRequester.html#tymethod.upload_files
#[inline]
pub fn upload_files(files: Vec<Vec<u8>>) -> Result<FileUploadResponse> {
    OwoClient::new(key)?.upload_files(files)
}

/// Shortens a URL via the service.
///
/// See [`OwoReqwestRequester`] for more information.
///
/// # Errors
///
/// Returns [`Error::NativeTls`] if there was an error instantiating the client.
///
/// [`Error::NativeTls`]: ../../../enum.Error.html#variant.NativeTls
/// [`OwoReqwestRequester`]: ../trait.OwoRequester.html
#[inline]
pub fn shorten_url(key: &str, url: &str) -> Result<String> {
    OwoClient::new(key)?.shorten_url(url)
}
