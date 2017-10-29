# OwO.rs

[![ci-badge][]][ci] [![docs-badge][]][docs]

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
with a key taken from an environment variable:

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

    // Retrieve the key from an environment variable named "OWO_KEY".
    let key = env::var("OWO_KEY").expect("OWO_KEY env missing");

    // Create the reqwest Client.
    //
    // In this case, we're using the client for a oneshot request, but in
    // normal cases a client can be re-used.
    let client = Client::new();

    let response = client.upload_file(&key, buffer).expect("Err in request");

    println!("Response: {:?}", response);
}
```

For more examples, refer to the [examples] folder or the [documentation][docs].

### Contributing

Pull requests are accepted. Make sure you add test suites for new features and
make sure the code passes the spec (so the build doesn't break). Tests are
automatically run when commits are made in a pull request.

### License

The contents of this repository are licensed under the MIT license. A copy of
the MIT license can be found in [LICENSE.md].

[ci]: https://travis-ci.org/whats-this/owo.rs
[ci-badge]: https://travis-ci.org/whats-this/owo.rs.svg?branch=master
[docs]: https://docs.rs/owo
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg
[examples]: https://github.com/whats-this/owo.rs/blob/master/examples
[LICENSE.md]: https://github.com/whats-this/owo.rs/blob/master/LICENSE.md
