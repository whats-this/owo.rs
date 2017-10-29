//! A collection of oneshot functions for performing requests over the API.
//!
//! These are functions that create a oneshot [`OwoHyperClient`]. Use at your
//! own expense. Oneshots are more expensive with repeated use.
//!
//! [`OwoHyperClient`]: ../struct.OwoClient.html

use hyper::client::FutureResponse;
use super::OwoClient;
use tokio_core::reactor::Handle;
use ::Result;

/// Shortens a URL via the service.
///
/// See [`OwoHyperRequester`] for more information.
///
/// # Errors
///
/// Returns [`Error::NativeTls`] if there was an error instantiating the client.
///
/// [`Error::NativeTls`]: ../../../enum.Error.html#variant.NativeTls
/// [`OwoHyperRequester`]: ../trait.OwoRequester.html
#[inline]
pub fn shorten_url(key: &str, url: &str, handle: &Handle)
    -> Result<FutureResponse> {
    OwoClient::new(key, handle)?.shorten_url(url)
}
