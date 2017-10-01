//! Bridge to provide client implementation for the `hyper` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`OwoRequester`].
//!
//! [`OwoRequester`]: trait.OwoRequester.html

use hyper::client::{Client as HyperClient, FutureResponse, HttpConnector};
use hyper::{Body, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use std::str::FromStr;
use ::Result;

/// Trait which defines the methods necessary to interact with the service.
///
/// # Exmaples
///
/// To bring in the implementation for the `reqwest` client, simply use the
/// trait:
///
/// ```rust,no_run
/// use owo::OwoHyperRequester;
/// ```
///
/// At this point, the methods will be on your Hyper Client.
pub trait OwoRequester {
    /// Shortens a URL via the service.
    ///
    /// # Examples
    ///
    /// Shorten the URL `"https://google.com"` via the service, using an
    /// environment variable for the key:
    ///
    /// ```rust,ignore
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate owo;
    /// extern crate tokio_core;
    ///
    /// use hyper_tls::HttpsConnector;
    /// use owo::OwoHyperRequester;
    /// use hyper::Client;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let key = env::var("OWO_TOKEN")?;
    ///
    /// let mut core = Core::new()?;
    ///
    /// let connector = HttpsConnector::new(4, &core.handle())?;
    /// let client = Client::configure()
    ///     .connector(connector)
    ///     .build(&core.handle());
    ///
    /// let url_to_shorten = "https://google.com";
    ///
    /// let runner = client.shorten_url(&key, url_to_shorten)?
    ///     .and_then(|res| {
    ///         res.body().for_each(|chunk| {
    ///             io::stdout().write_all(&chunk).map_err(From::from)
    ///         })
    ///     }).map(|_| {
    ///         println!("\n\nDone.");
    ///     });
    ///
    /// core.run(runner)?;
    /// ```
    ///
    // Note: This doc example can not be tested due to the reliance on
    // tokio_core. Instead, this is taken from example `01_hyper` and should
    // roughly match it to ensure accuracy.
    fn shorten_url(&self, key: &str, url: &str) -> Result<FutureResponse>;
}

impl OwoRequester for HyperClient<HttpsConnector<HttpConnector>, Body> {
    fn shorten_url(&self, key: &str, url: &str) -> Result<FutureResponse> {
        let req_url = format!(
            "https://api.awau.moe/shorten/polr?action=shorten&url={}&key={}",
            url,
            key,
        );
        let uri = Uri::from_str(&req_url)?;
        let request = Request::new(Method::Get, uri);

        Ok(self.request(request))
    }
}
