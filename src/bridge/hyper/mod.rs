//! Bridge to provide client implementation for the `hyper` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`OwoRequester`].
//!
//! [`OwoRequester`]: trait.OwoRequester.html

pub mod oneshot;

use hyper::client::{Client as HyperClient, FutureResponse, HttpConnector};
use hyper::header::UserAgent;
use hyper::{Body, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use std::str::FromStr;
use tokio_core::reactor::Handle;
use ::{constants, Result};

/// A light wrapper around a hyper Client, containing the client and the key to
/// use in requests.
///
/// This is a bit less inefficient than using your own client and storing your
/// own key elsewhere, due to this wrapper owning its own client and key. For
/// the best performance on memory, manage your own hyper Client for re-use
/// across multiple services.
///
/// Refer to [`OwoRequester`] for more information.
///
/// [`OwoRequester`]: trait.OwoRequester.html
pub struct OwoClient {
    client: HyperClient<HttpsConnector<HttpConnector>, Body>,
    /// The key in use by the client.
    pub key: String,
}

impl OwoClient {
    /// Creates a new client.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// extern crate owo;
    /// extern crate tokio_core;
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// use owo::OwoHyperClient;
    /// use std::env;
    /// use tokio_core::reactor::Core;
    ///
    /// let core = Core::new()?;
    /// let handle = core.handle();
    /// let client = OwoHyperClient::new(env::var("OWO_TOKEN")?, &handle);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::NativeTls`] if there was an error instantiating the
    /// HTTPS connector.
    ///
    /// [`Error::NativeTls`]: ../../enum.Error.html#variant.NativeTls
    pub fn new<S: Into<String>>(key: S, handle: &Handle) -> Result<Self> {
        let connector = HttpsConnector::new(4, handle)?;
        let client = HyperClient::configure()
            .connector(connector)
            .build(handle);

        Ok(Self {
            key: key.into(),
            client,
        })
    }

    /// Shortcut for shortening a URL.
    ///
    /// Refer to [`OwoRequester::upload_files`] for more information.
    ///
    /// # Examples
    ///
    /// Shorten the URL `"https://google.com"`, using a key from the
    /// environment:
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// use owo::OwoReqwestClient;
    /// use std::env;
    ///
    /// let client = OwoReqwestClient::new(env::var("OWO_TOKEN")?);
    ///
    /// println!("Response: {:?}", client.shorten_url("https://google.com")?);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// [`OwoRequester::upload_files`]: trait.OwoRequester.html#tymethod.upload_files
    #[inline]
    pub fn shorten_url(&self, url: &str) -> Result<FutureResponse> {
        self.client.shorten_url(&self.key, url)
    }
}

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
    /// extern crate futures;
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate owo;
    /// extern crate tokio_core;
    ///
    /// use futures::future::Future;
    /// use hyper::Client;
    /// use hyper_tls::HttpsConnector;
    /// use owo::OwoHyperRequester;
    /// use std::{env, io};
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
        let mut request = Request::new(Method::Get, uri);
        request.headers_mut().set(UserAgent::new(constants::USER_AGENT));

        Ok(self.request(request))
    }
}
