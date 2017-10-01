//! A sample application asking the user for an input URL to shorten.
//!
//! This makes use of a tokio reactor Core. Typically you would maintain a
//! single core for multiple concurrent events, but due to the oneshot nature of
//! this example, it's not particularly useful.

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate owo;
extern crate tokio_core;

use futures::{Future, Stream};
use hyper::Client as HyperClient;
use hyper_tls::HttpsConnector;
use owo::OwoHyperRequester;
use std::io::{self, Write};
use tokio_core::reactor::Core;

// You would want to handle results in a non-oneshot context.
fn main() {
    // Read the user input. If the input ends up being empty, let them know the
    // process is ending as there's no work to do.
    //
    // This will ask two questions:
    //
    // - first, their API key to make the request
    // - second, the URL to shorten
    let key = ask("Input your API key:");
    let key_trimmed = key.trim();

    if key_trimmed.is_empty() {
        println!("No key given. Shutting down.");

        return;
    }

    let input = ask("Input the URL to shorten:");
    let input_trimmed = input.trim();

    if input_trimmed.is_empty() {
        println!("No URL given. Shutting down.");

        return;
    }

    let mut core = Core::new().expect("Unable to create reactor core");

    let connector = HttpsConnector::new(4, &core.handle())
        .expect("Unable to create connector");
    let client = HyperClient::configure()
        .connector(connector)
        .build(&core.handle());

    let runner = client.shorten_url(&key_trimmed, &input_trimmed)
        .expect("Error making request")
        .and_then(|res| {
            res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        }).map(|_| {
            println!("\n\nDone.");
        });

    core.run(runner).expect("Error on core");
}

/// Asks the user a question and reads from stdin, returning the resultant
/// String buffer.
///
/// # Examples
///
/// Ask the user "What colour is the sky?" and print the result:
///
/// ```rust,no_run
/// println!("{}", ask("what colour is the sky?"));
/// ```
fn ask(question: &str) -> String {
    print!("{}\n>", question);
    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error processing input");

    input
}
