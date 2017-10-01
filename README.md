# owo.rs

A Rust client for the owo.whats-th.is API, offering implementations for both
asynchronous hyper (v0.11) and synchronous reqwest (v0.7).

### Compile Features

- **hyper-support**: Compiles with `hyper` support
- **reqwest-support**: Compiles with `reqwest` support (*default*)

**note**: `hyper` support is minimal due to lack of existing ecosystem multipart
support, and is currently restricted to URL shortening

### Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
owo = { git = "https://github.com/whats-this/owo.rs" }
```

To enable both `hyper` and `reqwest` support:

```toml
[dependencies.owo]
git = "https://github.com/whats-this/owo.rs"
features = ["hyper-support", "reqwest-support"]
```

To enable `hyper` but not `reqwest` support:

```toml
[dependencies.owo]
default-features = false
git = "https://github.com/whats-this/owo.rs"
features = ["hyper-support"]
```

### Examples

Using reqwest, upload a file by its filepath as a string taken from user input,
with a token taken from an environment variable:

```rust
extern crate owo;
extern crate reqwest;

use owo::OwoReqwestRequester;
use reqwest::Client;
use std::fs::File;
use std::io::{self, Read, Write};
use std::env;

fn main() {
    // Read a filepath to upload from user input:
    let mut path = String::new();
    print!("Enter the path on the filesystem to upload:\n>");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut path);

    // Read the file to a buffer of bytes
    let mut buffer = vec![];
    let mut file = File::open(path).expect("Error opening file");
    file.read_to_end(&mut buffer).expect("Error reading file");

    // Retrieve the token from an environment variable named "OWO_TOKEN".
    let token = env::var("OWO_TOKEN").expect("OWO_TOKEN env missing");

    // Create the reqwest Client.
    //
    // In this case, we're using the client for a oneshot request, but in
    // normal cases a client can be re-used.
    let client = Client::new();

    let response = client.upload_file(&token, buffer).expect("Err in request");

    println!("Response: {:?}", response);
}
```

For more examples, refer to the [examples] folder or the [documentation].

### License

ISC.

[documentation]: https://docs.rs/owo
[examples]: https://github.com/whats-this/owo.rs/blob/master/examples
