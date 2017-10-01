//! Bridge to provide a client implementation for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the documentation for [`OwoRequester`].
//!
//! [`OwoRequester`]: trait.OwoRequester.html

use reqwest::multipart::{Form, Part};
use reqwest::Client;
use serde_json;
use std::io::{Cursor, Read};
use ::model::FileUploadResponse;
use ::{Error, Result, constants};

/// Trait which defines the methods necessary to interact with the service.
///
/// # Examples
///
/// To bring in the implementation for the `reqwest` Client, simply use the
/// trait:
///
/// ```rust,no_run
/// use owo::OwoReqwestRequester;
/// ```
///
/// At this point, the methods will be on your Reqwest Client.
pub trait OwoRequester {
    /// Uploads a single file to the service.
    ///
    /// # Examples
    ///
    /// Upload a file by its filepath as a string taken from user input, with a
    /// token taken from an environment variable:
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// #
    /// extern crate owo;
    /// extern crate reqwest;
    ///
    /// use owo::OwoReqwestRequester;
    /// use reqwest::Client;
    /// use std::fs::File;
    /// use std::io::{self, Read, Write};
    /// use std::env;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// // Read a filepath to upload from user input:
    /// let mut input = String::new();
    /// print!("Enter the path on the filesystem to upload:\n>");
    /// io::stdout().flush()?;
    /// io::stdin().read_to_string(&mut input)?;
    ///
    /// // Read the content of the file to a buffer.
    /// let mut file = File::open(&input)?;
    /// let mut buffer = vec![];
    /// file.read_to_end(&mut buffer)?;
    ///
    /// // Retrieve the token from an environment variable named "OWO_TOKEN".
    /// let token = env::var("OWO_TOKEN")?;
    ///
    /// // Create the reqwest Client.
    /// //
    /// // In this case, we're using the client for a oneshot request, but in
    /// // normal cases a client can be re-used.
    /// let client = Client::new();
    ///
    /// let response = client.upload_file(&token, buffer);
    ///
    /// println!("Response: {:?}", response);
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
    /// Returns an [`Error::Reqwest`] if building the request fails.
    ///
    /// [`Error::Reqwest`]: ../../enum.Error.html#variant.Reqwest
    fn upload_file(&self, key: &str, file: Vec<u8>)
        -> Result<FileUploadResponse>;

    /// Uploads an array of files to the service, one-by-one.
    ///
    /// # Examples
    ///
    /// Upload two files to the service, one as a slice of bytes and the other
    /// as a path on the filesystem, using a variable from the environment:
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// #
    /// extern crate owo;
    /// extern crate reqwest;
    ///
    /// use owo::OwoReqwestRequester;
    /// use reqwest::Client;
    /// use std::env;
    /// use std::fs::File;
    /// use std::io::Read;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// let token = env::var("OWO_TOKEN")?;
    /// let client = Client::new();
    ///
    /// let mut cat1 = File::open("/mnt/media/images/cat1.png")?;
    /// let mut buffer1 = vec![];
    /// cat1.read_to_end(&mut buffer1)?;
    ///
    /// let mut cat2 = File::open("/mnt/media/images/cat2.png")?;
    /// let mut buffer2 = vec![];
    /// cat2.read_to_end(&mut buffer2)?;
    ///
    /// let responses = client.upload_files(&token, vec![buffer1, buffer2]);
    ///
    /// for (idx, response) in responses.iter().enumerate() {
    ///     println!("#{} response: {:?}", idx, response);
    /// }
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
    /// Returns an [`Error::Reqwest`] if building the request fails.
    ///
    /// [`Error::Reqwest`]: ../../enum.Error.html#variant.Reqwest
    fn upload_files(&self, key: &str, files: Vec<Vec<u8>>)
        -> Result<FileUploadResponse>;

    /// Shortens a URL via the service, returning a URL to the shortened link.
    ///
    /// # Examples
    ///
    /// Shorten the URL `"https://google.com"` via the service, using an
    /// environment variable for the key:
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// #
    /// extern crate owo;
    /// extern crate reqwest;
    ///
    /// use owo::OwoReqwestRequester;
    /// use reqwest::Client;
    /// use std::env;
    ///
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// let token = env::var("OWO_TOKEN")?;
    /// let client = Client::new();
    ///
    /// let url_to_shorten = "https://google.com";
    ///
    /// let url = client.shorten_url(&token, url_to_shorten)?;
    ///
    /// println!("url: {}", url);
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    fn shorten_url(&self, key: &str, url: &str) -> Result<String>;
}

impl OwoRequester for Client {
    fn upload_file(&self, key: &str, file: Vec<u8>)
        -> Result<FileUploadResponse> {
        let uri = format!("https://api.awau.moe/upload/pomf?key={}", key);

        let part = Part::reader(Cursor::new(file));
        let form = Form::new().part("files[]", part);

        upload(self, &uri, form)
    }

    fn upload_files(&self, key: &str, files: Vec<Vec<u8>>)
        -> Result<FileUploadResponse> {
        // Check that the number of requested files to upload is not too many.
        if files.len() > constants::MAX_FILES {
            return Err(Error::TooManyFiles);
        }

        let uri = format!("https://api.awau.moe/upload/pomf?key={}", key);

        let mut form = Form::new();

        for file in files {
            form = form.part("files[]", Part::reader(Cursor::new(file)));
        }

        upload(self, &uri, form)
    }

    fn shorten_url(&self, key: &str, url: &str) -> Result<String> {
        let uri = format!(
            "https://api.awau.moe/shorten/polr?action=shorten&url={}&key={}",
            url,
            key,
        );

        let mut response = self.get(&uri).send()?;
        let mut buffer = String::new();
        response.read_to_string(&mut buffer)?;

        Ok(buffer)
    }
}

fn upload(client: &Client, uri: &str, form: Form)
    -> Result<FileUploadResponse> {
    let reader = client.post(uri).multipart(form).send()?;

    serde_json::from_reader(reader).map_err(From::from)
}
