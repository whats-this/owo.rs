# owo.rs examples

This directory contains one example for each supported HTTP client.

All examples work in the fashion of first either asking you to provide an API
key - or retrieving it from an environment variable - and then either asking you
to provide a filepath to upload a file or provide a URL to shorten.

To compile and run an example, clone this repository and cd to the appropriate
example relevant to your HTTP client requirements, like so:

```sh
$ git clone https://github.com/whats-this/owo.rs
$ cd owo.rs/examples/02_reqwest
```

Then, run the project, which will compile the project, download and compile
all of its dependencies, and then run it:

```sh
$ cargo run
```

In the case of example 02, you must provide an `OWO_TOKEN` environment variable.
