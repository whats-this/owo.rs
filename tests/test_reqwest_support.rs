#![cfg(feature = "reqwest-support")]

extern crate owo;
extern crate reqwest;

use owo::OwoReqwestRequester;
use reqwest::Client;
use std::fs::File;
use std::io::Read;
use std::env;

fn read(relative_path: &str) -> Vec<u8> {
    let mut buffer = vec![];
    let mut file = File::open(relative_path).expect("err opening file");
    file.read_to_end(&mut buffer).expect("err reading file");

    buffer
}

#[inline]
fn key() -> String {
    env::var("OWO_KEY").expect("OWO_KEY env var not present")
}

#[ignore]
#[test]
fn test_upload_file() {
    let key = key();
    let client = Client::new();

    let buffer = read("tests/resources/cat.png");

    let res = client.upload_file(&key, buffer).expect("file err");

    assert!(res.success);
    assert_eq!(res.files[0].name, None);
    assert!(res.files[0].size > 1);
}

#[ignore]
#[test]
fn test_upload_files() {
    let key = key();
    let client = Client::new();

    let buffer1 = read("tests/resources/cat.png");
    let buffer2 = read("tests/resources/horse.png");

    let res = client.upload_files(&key, vec![buffer1, buffer2]).unwrap();

    assert!(res.success);
    assert!(res.files.len() == 2);

    for file in res.files {
        assert_eq!(file.name, None);
        assert!(file.size > 1);
    }
}

#[ignore]
#[test]
fn test_shorten_url() {
    let key = key();
    let client = Client::new();

    let res = client.shorten_url(&key, "https://google.com")
        .expect("shorten err");

    assert!(res.len() > 1);
    assert!(res.split('/').collect::<Vec<_>>().len() > 2);
}
