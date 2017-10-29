use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::result::Result as StdResult;

#[cfg(feature = "hyper")]
use hyper::error::UriError;
#[cfg(feature = "native-tls")]
use native_tls::Error as NativeTlsError;
#[cfg(feature = "reqwest")]
use std::io::Error as IoError;
#[cfg(feature = "serde_json")]
use serde_json::Error as JsonError;
#[cfg(feature = "reqwest")]
use reqwest::Error as ReqwestError;

/// A result type to compose a successful value and the library's [`Error`]
/// type.
///
/// [`Error`]: enum.Error.html
pub type Result<T> = StdResult<T, Error>;

/// An error type to compose a singular error enum between various dependencies'
/// errors.
#[derive(Debug)]
pub enum Error {
    /// An error from the `std::io` module.
    #[cfg(feature = "reqwest")]
    Io(IoError),
    /// An error from the `serde_json` crate.
    ///
    /// A potential reason for this is when there is an error deserializing a
    /// JSON response body.
    #[cfg(feature = "serde_json")]
    Json(JsonError),
    /// An error from the `native-tls` crate.
    #[cfg(feature = "native-tls")]
    NativeTls(NativeTlsError),
    /// An error from the `reqwest` crate when it is enabled.
    #[cfg(feature = "reqwest")]
    Reqwest(ReqwestError),
    /// Indicator that a request would have attempted to upload too many files.
    ///
    /// Refer to [`constants::MAX_FILES`] for the maximum number of allowed
    /// files per request.
    ///
    /// [`constants::MAX_FILES`]: constants/const.MAX_FILES.html
    TooManyFiles,
    /// An error when building a request's URI from the `hyper` crate when it is
    /// enabled.
    #[cfg(feature = "hyper")]
    Uri(UriError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            #[cfg(feature = "reqwest")]
            Error::Io(ref inner) => inner.fmt(f),
            #[cfg(feature = "serde_json")]
            Error::Json(ref inner) => inner.fmt(f),
            #[cfg(feature = "native-tls")]
            Error::NativeTls(ref inner) => inner.fmt(f),
            #[cfg(feature = "reqwest")]
            Error::Reqwest(ref inner) => inner.fmt(f),
            Error::TooManyFiles => f.write_str("Too many files to upload"),
            #[cfg(feature = "hyper")]
            Error::Uri(ref inner) => inner.fmt(f),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            #[cfg(feature = "reqwest")]
            Error::Io(ref inner) => inner.description(),
            #[cfg(feature = "serde_json")]
            Error::Json(ref inner) => inner.description(),
            #[cfg(feature = "native-tls")]
            Error::NativeTls(ref inner) => inner.description(),
            #[cfg(feature = "reqwest")]
            Error::Reqwest(ref inner) => inner.description(),
            Error::TooManyFiles => "Too many files to upload",
            #[cfg(feature = "hyper")]
            Error::Uri(ref inner) => inner.description(),
        }
    }
}

#[cfg(feature = "native-tls")]
impl From<NativeTlsError> for Error {
    fn from(err: NativeTlsError) -> Error {
        Error::NativeTls(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

#[cfg(feature = "serde_json")]
impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}

#[cfg(feature = "reqwest")]
impl From<ReqwestError> for Error {
    fn from(err: ReqwestError) -> Error {
        Error::Reqwest(err)
    }
}

#[cfg(feature = "hyper")]
impl From<UriError> for Error {
    fn from(err: UriError) -> Error {
        Error::Uri(err)
    }
}
